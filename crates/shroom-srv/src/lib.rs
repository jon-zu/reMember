#![allow(
    clippy::unused_async,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]


pub const MS_PER_TICK: u64 = 50;
pub const MSG_LIMIT_PER_TICK: usize = 100;

pub trait Id: Copy + Eq + std::hash::Hash + std::fmt::Debug + Send + 'static {}
impl<T: Copy + Eq + std::hash::Hash + std::fmt::Debug + Send + 'static> Id for T {}


pub mod act;

pub mod net {
    pub mod acceptor;
    pub mod session;
    pub mod socket;
    pub mod system;
}

pub mod game;

pub mod time {
    pub mod clock;
    pub mod interval;
}

pub use time::clock::{Clock, ClockHandle, Instant};

pub mod util {
    pub mod delay_queue;
    pub mod poll_state;
    pub mod supervised_task;
    //pub mod encode_buffer;
    //pub mod broadcast;

    pub use delay_queue::DelayQueue;
    //pub use encode_buffer::EncodeBuf;
    pub use supervised_task::{SupervisedTask, SupervisedTaskHandle};
}

pub mod rpc;
pub mod runtime;
pub mod session;


pub type GameTime = Instant;