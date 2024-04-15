use super::clock::{Instant, Ticks};

/// A simple interval, which runs every `period` ticks
#[derive(Debug)]
pub struct Interval {
    last_update: Option<Instant>,
    period: Ticks,
}

impl Interval {

    /// Creates a new interval, 
    #[must_use]
    pub fn new(t: Instant, ticks: Ticks) -> Self {
        Self {
            last_update: Some(t),
            period: ticks,
        }
    }


    /// Creates a new interval, 
    /// which initialize on the first try_tick call
    #[must_use]
    pub fn new_next(ticks: Ticks) -> Self {
        Self {
            last_update: None,
            period: ticks,
        }
    }

    /// Creates a interval from a duration
    #[must_use]
    pub fn from_dur(t: Instant, duration: std::time::Duration) -> Self {
        Self::new(t, duration.into())
    }

    /// Creates a interval from a duration
    #[must_use]
    pub fn from_dur_next(duration: std::time::Duration) -> Self {
        Self::new_next(duration.into())
    }

    /// Reset the interval to the given time
    pub fn reset(&mut self, t: Instant) {
        self.last_update = Some(t);
    }

    /// Attempts to tick the interval, returning true if it was successful
    #[must_use]
    pub fn try_tick(&mut self, t: Instant) -> bool {
        if self.last_update.get_or_insert(t).delta_ticks(t) >= self.period {
            self.last_update = Some(t);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Clock;

    use super::*;

    #[tokio::test]
    async fn interval() {
        let mut clock = Clock::default();
        let mut iv = Interval::new(clock.time(), Ticks(1));

        // Wait an initial tick
        clock.tick().await;

        assert!(iv.try_tick(clock.time()));
        assert!(!iv.try_tick(clock.time()));

        // Wait one tick
        clock.tick().await;

        assert!(iv.try_tick(clock.time()));
        assert!(!iv.try_tick(clock.time()));
    }
}
