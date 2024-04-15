pub mod room;
pub mod session;
pub mod sys;
pub mod acceptor;
pub mod runtime;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("broadcast error")]
    Broadcast,
    #[error("Send tx error")]
    SendTx,
    #[error("Recv tx error")]
    Recv,
    #[error("Session not found")]
    SessionNotFound,
    #[error("Room Broadcast")]
    RoomBroadcast(#[from] tokio::sync::broadcast::error::RecvError),
}
