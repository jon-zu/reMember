use crate::{ClockHandle, Instant};

pub trait TickHandler: Sized + Send + 'static {
    type Error: std::fmt::Debug + Send + 'static;

    fn on_tick(&mut self, t: Instant) -> Result<(), Self::Error>;
}

pub struct TickRunner<T> {
    clock_handle: ClockHandle,
    handler: T,
}

impl<T: TickHandler> TickRunner<T> {
    pub fn new(handler: T, handle: ClockHandle) -> Self {
        Self {
            clock_handle: handle,
            handler,
        }
    }

    pub async fn run(mut self) -> Result<(), T::Error> {
        loop {
            self.clock_handle.tick().await;
            self.handler.on_tick(self.clock_handle.time())?;
        }
    }
}
