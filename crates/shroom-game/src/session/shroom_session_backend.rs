use dashmap::DashSet;
use shroom_data::services::account::{AccountId, AccountServiceError};
use shroom_meta::id::CharacterId;
use thiserror::Error;

use shroom_srv::session::Backend;

use shroom_data::entities::{self, character};

use crate::{life::char::Character, services::shared::SharedGameServices};

#[derive(Debug, Error)]
pub enum ShroomSessionError {
    #[error("Invalid login session")]
    InvalidLoginSession,
    #[error("Invalid game session")]
    InvalidGameSession,
    #[error("Char not belonging to account")]
    CharNotBelongingToAccount,
    #[error("Account error: {0:?}")]
    Account(#[from] AccountServiceError),
    #[error("Other error: {0:?}")]
    Other(anyhow::Error),
}

#[derive(Debug)]
pub enum AccountAuth {
    UsernamePassword(String, String),
    Token(AccountId, CharacterId, [u8; 32]),
}

#[derive(Debug)]
pub struct SessionIngameData {
    pub acc: entities::account::Model,
    pub char: Character,
}

#[derive(Debug)]
pub struct SessionLoginData {
    pub acc: entities::account::Model,
}

#[derive(Debug)]
pub enum ShroomSessionData {
    Ingame(SessionIngameData),
    Login(SessionLoginData),
}

impl<'a> TryFrom<&'a mut ShroomSessionData> for &'a mut SessionIngameData {
    type Error = ShroomSessionError;

    fn try_from(value: &'a mut ShroomSessionData) -> Result<Self, Self::Error> {
        match value {
            ShroomSessionData::Ingame(ingame) => Ok(ingame),
            _ => Err(ShroomSessionError::InvalidGameSession),
        }
    }
}

impl<'a> TryFrom<&'a mut ShroomSessionData> for &'a mut SessionLoginData {
    type Error = ShroomSessionError;

    fn try_from(value: &'a mut ShroomSessionData) -> Result<Self, Self::Error> {
        match value {
            ShroomSessionData::Login(login) => Ok(login),
            _ => Err(ShroomSessionError::InvalidLoginSession),
        }
    }
}

impl ShroomSessionData {
    pub async fn transition_ingame(
        &mut self,
        char: character::Model,
        svc: &SharedGameServices,
    ) -> Result<(), ShroomSessionError> {
        let Self::Login(login) = self else {
            return Err(ShroomSessionError::InvalidLoginSession);
        };
        let acc_id = login.acc.id;
        let char_id = CharacterId(char.id as u32);
        if char.acc_id != acc_id {
            return Err(ShroomSessionError::CharNotBelongingToAccount);
        }

        let t = svc.current_time.load();
        let char = Character::new(
            svc.clone(),
            t,
            svc.data
                .char()
                .must_get(char_id)
                .await
                .map_err(ShroomSessionError::Other)?,
            svc.data
                .item
                .load_inventory_for_character(char_id)
                .await
                .map_err(ShroomSessionError::Other)?,
            svc.data
                .char()
                .load_skills(t, char_id) //TODO t
                .await
                .map_err(ShroomSessionError::Other)?,
            svc.data
                .char()
                .load_key_map(char_id)
                .await
                .map_err(ShroomSessionError::Other)?,
            svc.data
                .char()
                .load_quests(char_id)
                .await
                .map_err(ShroomSessionError::Other)?,
        );

        *self = Self::Ingame(SessionIngameData {
            acc: login.acc.clone(),
            char,
        });

        Ok(())
    }

    fn get_account(&self) -> &entities::account::Model {
        match self {
            Self::Ingame(ingame) => &ingame.acc,
            Self::Login(login) => &login.acc,
        }
    }
}

#[derive(Debug)]
pub struct ShroomSessionBackend {
    pub(crate) game: SharedGameServices,
    logged_in: DashSet<AccountId>,
}

impl ShroomSessionBackend {
    pub fn new(game: SharedGameServices) -> Self {
        Self {
            game,
            logged_in: DashSet::new(),
        }
    }
}

impl Backend for ShroomSessionBackend {
    type Data = Box<ShroomSessionData>;
    type LoadParam = AccountAuth;
    type Error = ShroomSessionError;
    type TransitionInput = character::Model;

    async fn load(&self, param: Self::LoadParam) -> Result<Self::Data, ShroomSessionError> {
        match param {
            AccountAuth::UsernamePassword(username, password) => {
                let acc = self
                    .game
                    .data
                    .account
                    .try_login(&username, &password)
                    .await?;

                if !self.logged_in.insert(acc.id) {
                    return Err(AccountServiceError::AccountAlreadyLoggedIn.into());
                }

                Ok(Box::new(ShroomSessionData::Login(SessionLoginData { acc })))
            }
            AccountAuth::Token(acc_id, chr, _token) => {
                //TODO verify the token
                let acc = self.game.account.get(acc_id).await.unwrap().unwrap();
                if !self.logged_in.insert(acc_id) {
                    return Err(AccountServiceError::AccountAlreadyLoggedIn.into());
                }
                let chr = self.game.data.char().get(chr).await.unwrap().unwrap();
                if chr.acc_id != acc.id {
                    return Err(AccountServiceError::UsernameWrongChar.into());
                }

                let mut sess = Box::new(ShroomSessionData::Login(SessionLoginData { acc }));
                self.transition(&mut sess, chr).await?;
                Ok(sess)
            }
        }
    }

    async fn save(&self, session: &mut Self::Data) -> Result<(), ShroomSessionError> {
        log::info!("Saving session for account {}", session.get_account().id);

        match session.as_mut() {
            ShroomSessionData::Ingame(ingame) => {
                let char_id = ingame.char.id;
                let d = &self.game.data;
                d.item
                    .save_inventory(&mut ingame.char.inventory.invs, char_id)
                    .await
                    .map_err(ShroomSessionError::Other)?;
                d.char()
                    .save_skills(ingame.char.last_update, char_id, &ingame.char.skills)
                    .await
                    .map_err(ShroomSessionError::Other)?;
                d.char()
                    .save_key_map(char_id, &ingame.char.key_map)
                    .await
                    .map_err(ShroomSessionError::Other)?;
                let q = ingame.char.quests.to_data();
                d.char()
                    .save_quest(char_id, q)
                    .await
                    .map_err(ShroomSessionError::Other)?;
                d.char()
                    .save_char(ingame.char.db_model())
                    .await
                    .map_err(ShroomSessionError::Other)?;
            }
            ShroomSessionData::Login(_login) => {}
        };

        Ok(())
    }

    async fn close(&self, session: &mut Self::Data) -> Result<(), ShroomSessionError> {
        log::info!("Closing session for account {}", session.get_account().id);
        self.logged_in.remove(&session.get_account().id);

        Ok(())
    }

    async fn transition(
        &self,
        session: &mut Self::Data,
        input: Self::TransitionInput,
    ) -> Result<(), Self::Error> {
        Box::pin(session.transition_ingame(input, &self.game)).await?;
        Ok(())
    }
}
