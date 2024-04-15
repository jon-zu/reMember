use std::{fmt::Debug, net::IpAddr, time::Duration};

use uuid::Uuid;

use super::shroom_session_backend::{
    SessionIngameData, SessionLoginData, ShroomSessionData, ShroomSessionError,
};

use shroom_srv::session::{
    migration_manager::MigrationManager, session_manager::SessionManager, Backend, Error, OwnedMappedSession, OwnedSession
};

pub type OwnedShroomSession = OwnedSession<Uuid, Box<ShroomSessionData>>;

pub type OwnedShroomLoginSession =
    OwnedMappedSession<Uuid, Box<ShroomSessionData>, SessionLoginData>;
pub type OwnedShroomGameSession =
    OwnedMappedSession<Uuid, Box<ShroomSessionData>, SessionIngameData>;

// Client uses a 8 byte session id
pub type ClientKey = [u8; 8];

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct ShroomMigrationKey {
    client_key: ClientKey,
    peer_addr: IpAddr,
}

impl ShroomMigrationKey {
    pub fn new(client_key: ClientKey, peer_addr: IpAddr) -> Self {
        Self {
            client_key,
            peer_addr,
        }
    }
}

/// Manages all sessions
pub struct ShroomSessionManager<B: Backend> {
    session_man: SessionManager<Uuid, B>,
    migration: MigrationManager<ShroomMigrationKey, OwnedSession<Uuid, B::Data>>

}

impl<B: Backend + std::fmt::Debug> Debug for ShroomSessionManager<B>
where
    B::Data: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShroomSessionManager")
            .field("session_man", &self.session_man)
            .finish()
    }
}

impl<B> ShroomSessionManager<B>
where
    B: Backend<Error = ShroomSessionError> + std::fmt::Debug + Send + 'static,
    B::Data: std::fmt::Debug,
{
    pub fn new(backend: B, migration_timeout: Duration) -> Self {
        Self {
            session_man: SessionManager::new(backend),
            migration: MigrationManager::new(migration_timeout),
        }
    }

    fn gen_key() -> uuid::Uuid {
        Uuid::new_v4()
    }

    pub async fn next_dropped_session(&self) -> Uuid {
        self.session_man.next_dropped_session().await
    }

    pub async fn create_claimed_session(
        &self,
        param: B::LoadParam,
    ) -> Result<OwnedSession<uuid::Uuid, B::Data>, Error<ShroomSessionError>> {
        self.session_man
            .create_claimed_session(Self::gen_key(), param)
            .await
    }

    pub async fn clean(&self) -> anyhow::Result<()> {
        // Remove timed out migrations and free up the sessions
        self.migration.clean();

        // Clean up all un-owned sessions
        self.session_man.remove_unowned_session().await?;

        Ok(())
    }

    /// Attempts to close a session by id
    pub async fn close_session_by_key(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        let session = self.session_man.try_claim_session(id)?;
        self.close_session(session).await
    }

    /// Closes a the session
    pub async fn close_session(
        &self,
        session: OwnedSession<uuid::Uuid, B::Data>,
    ) -> anyhow::Result<()> {
        Ok(self.session_man.close_session(session).await?)
    }

    pub async fn transition_migrate_session(
        &self,
        migration_key: ShroomMigrationKey,
        mut session: OwnedSession<uuid::Uuid, B::Data>,
        param: B::TransitionInput,
    ) -> anyhow::Result<()> {
        self.transition_session(&mut session, param).await?;
        self.migrate_session(migration_key, session)?;

        Ok(())
    }

    /// Creates a new sessions, which is set into a migration state
    pub async fn transition_session(
        &self,
        session: &mut OwnedSession<uuid::Uuid, B::Data>,
        param: B::TransitionInput,
    ) -> anyhow::Result<()> {
        self.session_man.transition(session, param).await?;
        Ok(())
    }

    /// Puts a session into a migration state
    pub fn migrate_session(
        &self,
        migration_key: ShroomMigrationKey,
        session: OwnedSession<uuid::Uuid, B::Data>,
    ) -> anyhow::Result<()> {
        self.migration.insert(migration_key, session);
        Ok(())
    }

    /// Tries to claim a session in migration, with the given key
    pub async fn claim_migrating_session(
        &self,
        migration_key: ShroomMigrationKey,
    ) -> anyhow::Result<OwnedSession<uuid::Uuid, B::Data>> {
        self.migration
            .take_with_timeout(&migration_key, self.migration.timeout())
            .await
    }
}
