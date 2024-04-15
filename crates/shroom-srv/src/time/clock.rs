use std::{
    num::NonZeroU64,
    ops::Add,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};

use tokio::sync::Notify;

pub const MS_PER_TICK: u64 = 50;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Ticks(pub u64);

impl From<Duration> for Ticks {
    fn from(d: Duration) -> Self {
        Self((d.as_millis() / MS_PER_TICK as u128) as u64)
    }
}

impl From<Ticks> for Duration {
    fn from(t: Ticks) -> Self {
        Duration::from_millis(t.0 * MS_PER_TICK)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[repr(transparent)]
pub struct Instant(NonZeroU64);

impl Default for Instant {
    fn default() -> Self {
        Self(NonZeroU64::new(1).unwrap())
    }
}

impl Instant {
    #[must_use]
    pub fn add_ms(&self, ms: u64) -> Self {
        Self(self.0.saturating_add(ms))
    }

    #[must_use]
    pub fn ticks(&self) -> Ticks {
        Ticks(self.0.get() / MS_PER_TICK)
    }

    #[must_use]
    pub fn add_ticks(&self, ticks: Ticks) -> Self {
        self.add_ms(ticks.0 * MS_PER_TICK)
    }

    #[must_use]
    pub fn expired(&self, other: Self) -> bool {
        self.0 >= other.0
    }

    #[must_use]
    pub fn delta(&self, t: Self) -> u64 {
        t.0.get() - self.0.get()
    }

    #[must_use]
    pub fn delta_ticks(&self, t: Self) -> Ticks {
        Ticks(self.delta(t) / MS_PER_TICK)
    }

    #[must_use]
    pub fn as_millis(&self) -> u64 {
        self.0.get()
    }

    #[must_use]
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        if self.0 < earlier.0 {
            return None;
        }
        Some(Duration::from_millis(self.0.get() - earlier.0.get()))
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        self.add_ms(u64::try_from(rhs.as_millis()).unwrap())
    }
}

impl Add<Ticks> for Instant {
    type Output = Self;

    fn add(self, rhs: Ticks) -> Self::Output {
        self.add_ticks(rhs)
    }
}

#[derive(Debug)]
struct Shared {
    clock: AtomicU64,
    notify: Notify,
    start: tokio::time::Instant,
}

impl Shared {
    fn from_start(start: tokio::time::Instant) -> Self {
        Self {
            clock: AtomicU64::new(1),
            notify: Notify::new(),
            start,
        }
    }

    /// Load the current time
    fn current_time(&self) -> Instant {
        // TODO use crossbeam atomicell
        // # Safety clock is initialized with 1 and only incremented
        Instant(unsafe {
            NonZeroU64::new_unchecked(self.clock.load(std::sync::atomic::Ordering::SeqCst))
        })
    }

    /// Tick the clock internally
    /// this should be only called by the clock itself
    fn tick(&self) {
        self.clock
            .fetch_add(MS_PER_TICK, std::sync::atomic::Ordering::SeqCst);
        self.notify.notify_waiters();
    }

    /// Check if the clock is ahead of the given time
    fn is_ahead(&self, t: Instant) -> bool {
        t.0 < self.current_time().0
    }

    /// Wait for the next tick
    async fn wait_for_tick(&self, t: Instant) {
        // The clock is already ahead
        if self.is_ahead(t) {
            return;
        }

        // Build a notification handle for the next tick
        let notify = self.notify.notified();

        // Check if the clock ticked during the creating
        if t.0 < self.current_time().0 {
            return;
        }

        // Else we wait for the notification
        notify.await;
    }
}

#[derive(Debug)]
pub struct Clock(Arc<Shared>);

impl Default for Clock {
    fn default() -> Self {
        Self::from_start(tokio::time::Instant::now())
    }
}

impl Clock {
    /// Create a new clock from the given start time
    #[must_use]
    pub fn from_start(start: tokio::time::Instant) -> Self {
        Self(Arc::new(Shared::from_start(start)))
    }

    /// Calculate the duration until the next tick
    #[must_use]
    pub fn wait_duration(&self) -> Duration {
        let elapsed = tokio::time::Instant::now().duration_since(self.0.start);
        let next = Duration::from_millis(self.0.current_time().add_ms(MS_PER_TICK).as_millis());
        next.checked_sub(elapsed).unwrap_or_default()
    }

    /// Tick the clock
    pub async fn tick(&mut self) {
        tokio::time::sleep(self.wait_duration()).await;
        self.0.tick();
    }

    /// Creates a new `ClockHandle` at the current time
    #[must_use]
    pub fn handle(&mut self) -> ClockHandle {
        ClockHandle {
            shared: self.0.clone(),
            time: self.0.current_time(),
        }
    }

    /// Get the current time of the clock
    #[must_use]
    pub fn time(&self) -> Instant {
        self.0.current_time()
    }
}

/// A `ClockHandle` keeps it own current time
/// but uses a `Clock` as reference to keep up to It
#[derive(Debug)]
pub struct ClockHandle {
    shared: Arc<Shared>,
    time: Instant,
}

impl Clone for ClockHandle {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
            time: self.time,
        }
    }
}

impl ClockHandle {
    /// Get the current time of the clock handle
    #[must_use]
    pub fn time(&self) -> Instant {
        self.time
    }

    /// Get the current tick of the clock handle
    #[must_use]
    pub fn ticks(&self) -> Ticks {
        self.time.ticks()
    }

    /// Try to tick the clock handle
    pub fn try_tick(&mut self) -> bool {
        if !self.shared.is_ahead(self.time) {
            return false;
        }

        self.time = self.time.add_ms(MS_PER_TICK);
        true
    }

    /// Tick the clock handle
    /// this will wait for the next tick, If the clock did not tick yet
    pub async fn tick(&mut self) {
        self.shared.wait_for_tick(self.time).await;
        self.time = self.time.add_ms(MS_PER_TICK);
    }

    /*pub fn tick_stream(mut self) -> impl Stream<Item = Instant> {
        async_stream::stream! {
            loop {
                self.tick().await;
                yield self.time;
            }
        }
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn clock() {
        let mut clock = Clock::default();
        let mut handle = clock.handle();

        assert_eq!(handle.ticks(), Ticks(0));
        clock.tick().await;

        // Only clock ticked, handle did not
        assert_eq!(handle.ticks(), Ticks(0));
        handle.tick().await;
        // Handle should have ticked now
        assert_eq!(handle.ticks(), Ticks(1));

        // Clock ticks async
        tokio::spawn(async move {
            clock.tick().await;
        });

        // Let the handle ticks
        handle.tick().await;
        assert_eq!(handle.ticks(), Ticks(2));
    }

    #[tokio::test]
    async fn keep_up() {
        let mut clock = Clock::default();
        let mut handle = clock.handle();

        // Unable to tick per default
        assert!(!handle.try_tick());

        clock.tick().await;
        clock.tick().await;

        // Try to tick twice, then we would have to wait again
        assert!(handle.try_tick());
        assert!(handle.try_tick());
        assert!(!handle.try_tick());
    }
}
