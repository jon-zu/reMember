use std::panic::AssertUnwindSafe;

use dashmap::DashMap;
use futures::FutureExt;
use tokio::sync::{mpsc, Mutex};

use super::{Backend, Error, OwnedSession, SessionHandle, SessionKey};

// TODO
// * use last resort save + handle the scenario the backend fails

#[derive(Debug)]
pub struct SessionManager<Key: SessionKey, B: Backend> {
    sessions: DashMap<Key, SessionHandle<B::Data>>,
    backend: B,
    dropped_session_rx: Mutex<mpsc::Receiver<Key>>,
    dropped_session_tx: mpsc::Sender<Key>,
}

impl<Key, B> SessionManager<Key, B>
where
    Key: SessionKey,
    B: Backend,
{
    pub fn new(backend: B) -> Self {
        let (dropped_session_tx, dropped_session_rx) = mpsc::channel(128);
        Self {
            sessions: DashMap::new(),
            backend,
            dropped_session_rx: Mutex::new(dropped_session_rx),
            dropped_session_tx,
        }
    }

    pub fn session(&self) -> usize {
        self.sessions.len()
    }

    /// Gets the next dropped session
    pub async fn next_dropped_session(&self) -> Key {
        //TODO remove the lock
        self.dropped_session_rx.try_lock().expect("rx lock").recv().await.expect("drop rx")
    }

    /// Helper function to close a session
    async fn close_session_inner(&self, session_data: &mut B::Data) -> Result<(), Error<B::Error>> {
        // After the session is removed save It
        self.backend
            .save(session_data)
            .await
            .map_err(Error::Backend)?;

        self.backend
            .close(session_data)
            .await
            .map_err(Error::Backend)?;

        Ok(())
    }

    /// Closes a session, but catches potential panics
    /// and errors during the process to close the session
    async fn safe_close(&self, session_data: &mut B::Data) -> Result<(), Error<B::Error>> {
        let res = AssertUnwindSafe(self.close_session_inner(session_data))
            .catch_unwind()
            .await;
        match res {
            Ok(res) => res,
            Err(_) => Err(Error::SavePanic),
        }
    }

    fn create_session_with(
        &self,
        key: Key,
        f: impl FnOnce() -> SessionHandle<B::Data>,
    ) -> Result<(), Error<B::Error>> {
        let mut inserted = false;
        self.sessions.entry(key).or_insert_with(|| {
            inserted = true;
            f()
        });

        if !inserted {
            return Err(Error::SessionKeyAlreadyExists);
        }

        Ok(())
    }

    /// Gets  a shared session handle by key
    fn get_session(&self, key: &Key) -> Result<SessionHandle<B::Data>, Error<B::Error>> {
        Ok(self
            .sessions
            .get(key)
            .ok_or_else(|| Error::SessionKeyNotExists)?
            .value()
            .clone())
    }

    /// Transition a session into a new state, using the transition input
    pub async fn transition(
        &self,
        session: &mut OwnedSession<Key, B::Data>,
        input: B::TransitionInput,
    ) -> Result<(), Error<B::Error>> {
        self.backend
            .transition(session, input)
            .await
            .map_err(Error::Backend)?;

        Ok(())
    }

    /// Remove all un-owned session
    /// acts essentially like a life-cycle
    pub async fn remove_unowned_session(&self) -> Result<(), Error<B::Error>> {
        let mut removed = vec![];

        // Drain all un-owned sessions
        self.sessions.retain(|_, v| {
            if let Ok(lock) = v.clone().try_lock_owned() {
                removed.push((lock, v.clone()));
                false
            } else {
                true
            }
        });

        if !removed.is_empty() {
            log::info!("Removed {:?} un-owned sessions", removed.len());
        }

        // As we are the last holder of the session we also need to close them
        for (session_lock, session) in removed {
            // Drop the lock
            drop(session_lock);
            // We held the lock before removing it, so there's only this reference left
            let mut session = session.try_lock().expect("exclusive session lock");

            let res = self.safe_close(&mut session).await;
            if let Err(err) = res {
                log::error!("Error during saving Session: {err:?}");
            }
        }

        Ok(())
    }

    /// Closes an owned session
    pub async fn close_session(
        &self,
        mut owned_session: OwnedSession<Key, B::Data>,
    ) -> Result<(), Error<B::Error>> {
        // Remove session, If the session exist It must be in the map
        let (_, session) = self.sessions.remove(&owned_session.key).expect("session");
        // We are closing the session now, so we already got the drop msg
        owned_session.skip_drop_msg();
        // Release lock to decrement the ref count to 1
        drop(owned_session);
        // Now we can claim the session
        let mut session = session.try_lock().expect("close owned session");
        self.safe_close(&mut session).await
    }

    /// Create a sessions with the given key and the load parameter
    /// the session data will be fetched with the backend
    pub async fn create_session(
        &self,
        key: Key,
        param: B::LoadParam,
    ) -> Result<(), Error<B::Error>> {
        let data = self.backend.load(param).await.map_err(Error::Backend)?;
        self.create_session_with(key, || SessionHandle::new(Mutex::new(data)))
    }

    /// Creates a claimed session, like `create_session`
    /// but this will also claim the session after create the session
    pub async fn create_claimed_session(
        &self,
        key: Key,
        param: B::LoadParam,
    ) -> Result<OwnedSession<Key, B::Data>, Error<B::Error>> {
        // Load the session data
        let data = self.backend.load(param).await.map_err(Error::Backend)?;

        // We capture the claim in the closure
        let mut claimed = None;
        self.create_session_with(key.clone(), || {
            let session = SessionHandle::new(Mutex::new(data));
            claimed = Some(session.clone().try_lock_owned().expect("claim session"));
            session
        })?;

        // claimed must have been set
        Ok(OwnedSession::new(key, claimed.expect("claimed session"), self.dropped_session_tx.clone()))
    }

    /// Tries to claim a session for the given key
    pub fn try_claim_session(
        &self,
        key: Key,
    ) -> Result<OwnedSession<Key, B::Data>, Error<B::Error>> {
        let session = self.get_session(&key)?;

        Ok(OwnedSession::new(
            key,
            session
                .try_lock_owned()
                .map_err(|_| Error::UnableToLockSession)?,
            self.dropped_session_tx.clone()
        ))
    }
}
