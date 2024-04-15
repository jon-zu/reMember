use futures::Future;
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::Arc,
};

pub mod migration_manager;
pub mod session_manager;

/// Backend for the sessions, to load and save the session data
pub trait Backend {
    /// Session Data
    type Data;
    /// Parameter for the loading the session data
    type LoadParam;
    /// Input for a session transition
    type TransitionInput;
    /// Error type
    type Error: std::fmt::Debug;

    /// Loads the session data with the given parameter
    fn load(
        &self,
        param: Self::LoadParam,
    ) -> impl Future<Output = Result<Self::Data, Self::Error>> + Send;
    /// Saves the session data
    fn save(
        &self,
        session: &mut Self::Data,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// When saving fails this function will be called
    /// a good idea is to save the data to an error file so It's not lost
    #[allow(unused_variables)]
    fn last_resort_save(&self, session: &mut Self::Data) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Closes the session
    fn close(
        &self,
        session: &mut Self::Data,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Transition the session into a new state
    fn transition(
        &self,
        session: &mut Self::Data,
        input: Self::TransitionInput,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

/// Represents a session which is owned by this struct
#[derive(Debug)]
pub struct OwnedSession<Key: SessionKey, Data> {
    key: Key,
    session: tokio::sync::OwnedMutexGuard<Data>,
    drop_tx: tokio::sync::mpsc::Sender<Key>,
    skip_drop_msg: bool,
}

impl<Key: SessionKey, Data> Drop for OwnedSession<Key, Data> {
    fn drop(&mut self) {
        if !self.skip_drop_msg {
            let _ = self.drop_tx.try_send(self.key.clone());
        }
    }
}

impl<Key: SessionKey, Data> OwnedSession<Key, Data> {
    /// Create a new owned session, from the locked session and the key
    pub fn new(
        key: Key,
        session: tokio::sync::OwnedMutexGuard<Data>,
        drop_tx: tokio::sync::mpsc::Sender<Key>,
    ) -> Self {
        Self {
            key,
            session,
            drop_tx,
            skip_drop_msg: false,
        }
    }

    /// Skip the drop message
    pub fn skip_drop_msg(&mut self) {
        self.skip_drop_msg = true;
    }

    /// Obtain the key of the owned session
    pub fn key(&self) -> &Key {
        &self.key
    }

    /// Map the session to a specific value
    pub fn map<Mapped, F>(mut self, f: F) -> OwnedMappedSession<Key, Data, Mapped>
    where
        F: FnOnce(&mut Data) -> &mut Mapped,
    {
        let mapped = f(&mut self.session);
        OwnedMappedSession {
            mapped: unsafe { NonNull::new_unchecked(mapped as *mut Mapped) },
            session: self,
        }
    }

    /// Map the session to a specific value
    pub fn try_map<Mapped, F, E>(mut self, f: F) -> Result<OwnedMappedSession<Key, Data, Mapped>, E>
    where
        F: FnOnce(&mut Data) -> Result<&mut Mapped, E>,
    {
        let mapped = f(&mut self.session)?;
        Ok(OwnedMappedSession {
            mapped: unsafe { NonNull::new_unchecked(mapped as *mut Mapped) },
            session: self,
        })
    }
}

impl<Key: SessionKey, SessionData> Deref for OwnedSession<Key, SessionData> {
    type Target = SessionData;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl<Key: SessionKey, SessionData> DerefMut for OwnedSession<Key, SessionData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.session
    }
}

/// Represents a session which is owned,
/// but dereferences to the mapped value
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct OwnedMappedSession<K: SessionKey, T, U> {
    session: OwnedSession<K, T>,
    mapped: NonNull<U>,
}

impl<Key: SessionKey, Data, Mapped> OwnedMappedSession<Key, Data, Mapped> {
    /// Unmap the session, returning the owned session
    pub fn unmap(self) -> OwnedSession<Key, Data> {
        self.session
    }
}

impl<Key: SessionKey, Data, Mapped> Deref for OwnedMappedSession<Key, Data, Mapped> {
    type Target = Mapped;

    fn deref(&self) -> &Self::Target {
        unsafe { self.mapped.as_ref() }
    }
}

impl<Key: SessionKey, Data, Mapped> DerefMut for OwnedMappedSession<Key, Data, Mapped> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.mapped.as_mut() }
    }
}

/// Safety: When all types are Send so is the mapped session
unsafe impl<Key: SessionKey, Data, Mapped> Send for OwnedMappedSession<Key, Data, Mapped>
where
    Key: Send,
    Data: Send,
    Mapped: Send,
{
}

/// Safety: When all types are Sync so is the mapped session
unsafe impl<Key: SessionKey, Data, Mapped> Sync for OwnedMappedSession<Key, Data, Mapped>
where
    Key: Sync,
    Data: Sync,
    Mapped: Sync,
{
}

pub type SessionHandle<Data> = Arc<tokio::sync::Mutex<Data>>;

pub trait SessionKey: Eq + Hash + Clone + std::fmt::Debug {}
impl<T: Eq + Hash + Clone + std::fmt::Debug> SessionKey for T {}

#[derive(Debug, thiserror::Error)]
pub enum Error<BackendError> {
    #[error("backend error: {0:?}")]
    Backend(BackendError),
    #[error("panic occured during saving")]
    SavePanic,
    #[error("session key already exists")]
    SessionKeyAlreadyExists,
    #[error("unable to lock session")]
    UnableToLockSession,
    #[error("Session for key does not exist")]
    SessionKeyNotExists,
}

#[cfg(test)]
mod tests {
    use tests::session_manager::SessionManager;

    use super::*;

    #[derive(Debug)]
    pub struct MockSessionBackend;

    impl Backend for MockSessionBackend {
        type Data = usize;
        type LoadParam = usize;
        type TransitionInput = usize;
        type Error = anyhow::Error;

        async fn load(&self, param: Self::LoadParam) -> Result<Self::Data, Self::Error> {
            Ok(param)
        }
        async fn save(&self, _session: &mut Self::Data) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Closes the session
        async fn close(&self, _session: &mut Self::Data) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Transition the session into a new state
        async fn transition(
            &self,
            session: &mut Self::Data,
            input: Self::TransitionInput,
        ) -> Result<(), Self::Error> {
            *session += input;
            Ok(())
        }
    }

    #[tokio::test]
    async fn session_man() {
        let sm = SessionManager::<u32, MockSessionBackend>::new(MockSessionBackend);

        let mut sess = sm.create_claimed_session(1, 0).await.unwrap();
        assert_eq!(sess.deref(), &0);
        assert_eq!(sess.key, 1);
        assert!(sm.try_claim_session(1).is_err());

        sm.transition(&mut sess, 10).await.unwrap();
        assert_eq!(sess.deref(), &10);

        // No sessions should be removed
        sm.remove_unowned_session().await.unwrap();
        assert_eq!(sm.session(), 1);
        // Drop our session handle
        drop(sess);
        assert_eq!(sm.session(), 1);

        // Now the sessions should be removed
        sm.remove_unowned_session().await.unwrap();
        assert_eq!(sm.session(), 0);

        // Create a new session and close it properly
        let sess = sm.create_claimed_session(1, 0).await.unwrap();
        assert_eq!(sm.session(), 1);
        sm.close_session(sess).await.unwrap();
        assert_eq!(sm.session(), 0);
    }
}
