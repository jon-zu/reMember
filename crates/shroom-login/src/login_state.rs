use std::future::Future;

use shroom_data::{
    entities::{account, character},
    services::character::CharWithEquips,
};
use shroom_game::session::shroom_session_manager::OwnedShroomLoginSession;
use shroom_meta::id::CharacterId;
use shroom_proto95::login::{AccountInfo, ChannelId, ClientKey, WorldId};

#[derive(Debug, Clone, Default, PartialEq)]
enum LoginStage {
    #[default]
    Unauthorized,
    AcceptTOS,
    Pin,
    SetGender,
    ServerSelection,
    CharSelection {
        world: WorldId,
        channel: ChannelId,
        chars: Vec<CharWithEquips>,
    },
}

/// State for the login server
/// the core idea is that the whole login logic
/// is handled in the state and illegal operations
/// will result in an error
#[derive(Debug, Default)]
pub struct LoginState {
    stage: LoginStage,
    login_session: Option<OwnedShroomLoginSession>,
    client_key: Option<ClientKey>,
}

impl LoginState {
    pub fn new() -> Self {
        Self {
            stage: LoginStage::Unauthorized,
            login_session: None,
            client_key: None,
        }
    }

    fn check_stage(&self, stage: LoginStage) -> anyhow::Result<()> {
        if self.stage != stage {
            anyhow::bail!("Expected stage: {stage:?}, current stage: {:?}", self.stage);
        }

        Ok(())
    }

    /// Returns an immutable reference to the account
    fn get_account(&self) -> anyhow::Result<&account::Model> {
        self.login_session
            .as_ref()
            .map(|login| &login.acc)
            .ok_or_else(|| anyhow::format_err!("Not authorized"))
    }

    pub fn get_client_key(&self) -> anyhow::Result<ClientKey> {
        self.client_key
            .ok_or_else(|| anyhow::anyhow!("No client key"))
    }

    /// Claim account so It can not be used by the state any longer
    pub fn claim_session(&mut self) -> anyhow::Result<OwnedShroomLoginSession> {
        self.stage = LoginStage::Unauthorized;
        Ok(self.login_session.take().unwrap())
    }

    pub fn reset(&mut self) {
        self.reset_replace();
    }

    fn reset_replace(&mut self) -> LoginStage {
        self.client_key = None;
        std::mem::take(&mut self.stage)
    }

    pub fn transition_game(
        &mut self,
        selected_char_id: CharacterId,
    ) -> anyhow::Result<(character::Model, ClientKey, WorldId, ChannelId)> {
        // Verify stage
        let LoginStage::CharSelection { ref chars, .. } = self.stage else {
            anyhow::bail!(
                "Expected stage: CharSelect, current stage: {:?}",
                self.stage
            );
        };

        // Find char_ix
        let Some(char_ix) = chars
            .iter()
            .position(|c| c.char.id as u32 == selected_char_id.0)
        else {
            anyhow::bail!("Invalid char id: {selected_char_id}");
        };

        // Claim old state
        let client_key = self.client_key.expect("client key");
        let LoginStage::CharSelection {
            chars,
            world,
            channel,
        } = self.reset_replace()
        else {
            anyhow::bail!(
                "Expected stage: CharSelect, current stage: {:?}",
                self.stage
            );
        };
        // We now the char list contains the char at the index
        let char = chars.into_iter().nth(char_ix).unwrap();
        Ok((char.char, client_key, world, channel))
    }

    pub fn get_char_select(
        &self,
    ) -> anyhow::Result<(&account::Model, WorldId, ChannelId, &[CharWithEquips])> {
        if let LoginStage::CharSelection {
            world,
            channel,
            chars,
        } = &self.stage
        {
            return Ok((
                self.get_account().expect("Account"),
                *world,
                *channel,
                &chars,
            ));
        }

        anyhow::bail!(
            "Expected stage: CharSelect, current stage: {:?}",
            self.stage
        );
    }

    pub fn get_pin(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::Pin)?;
        self.get_account()
    }

    pub fn get_set_gender(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::SetGender)?;
        self.get_account()
    }

    pub fn get_accept_tos(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::AcceptTOS)?;
        self.get_account()
    }

    pub fn get_unauthorized(&self) -> anyhow::Result<()> {
        self.check_stage(LoginStage::Unauthorized)?;
        Ok(())
    }

    pub fn get_server_selection(&self) -> anyhow::Result<&account::Model> {
        self.check_stage(LoginStage::ServerSelection)
            .or_else(|_| self.check_stage(LoginStage::Pin))?; //Char select
        self.get_account()
    }

    /// Updates the account with the given update operation
    /// ensures that database and local model are in-sync
    pub async fn update_account<F, Fut>(&mut self, update: F) -> anyhow::Result<&account::Model>
    where
        F: FnOnce(account::Model) -> Fut,
        Fut: Future<Output = anyhow::Result<account::Model>>,
    {
        let login = self
            .login_session
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;

        let new_acc = update(login.acc.clone()).await?;
        login.acc = new_acc;
        Ok(&login.acc)
    }

    pub fn is_accept_tos_stage(&self) -> bool {
        matches!(self.stage, LoginStage::AcceptTOS)
    }

    pub fn is_set_gender_stage(&self) -> bool {
        matches!(self.stage, LoginStage::SetGender)
    }

    pub fn is_unauthorized_stage(&self) -> bool {
        matches!(self.stage, LoginStage::Unauthorized)
    }

    /// Transitions the stage with the given account
    pub fn transition_login_with_session(
        &mut self,
        session: OwnedShroomLoginSession,
    ) -> anyhow::Result<()> {
        self.client_key = Some((session.acc.id as u64).to_le_bytes());
        let has_gender = session.acc.gender.is_some();
        let accepted_tos = session.acc.accepted_tos;
        self.stage = if !accepted_tos {
            LoginStage::AcceptTOS
        } else if !has_gender {
            LoginStage::SetGender
        } else {
            LoginStage::Pin
        };

        self.login_session = Some(session);

        Ok(())
    }

    pub fn transition_char_select(
        &mut self,
        world: WorldId,
        channel: ChannelId,
        chars: Vec<CharWithEquips>,
    ) -> anyhow::Result<()> {
        self.stage = LoginStage::CharSelection {
            world,
            channel,
            chars,
        };
        Ok(())
    }

    pub fn transition_server_select(&mut self) -> anyhow::Result<()> {
        self.stage = LoginStage::ServerSelection;
        Ok(())
    }

    pub fn get_account_info(&self) -> anyhow::Result<AccountInfo> {
        Ok(self.get_account()?.into())
    }
}
