use std::net::IpAddr;

use constant_time_eq::constant_time_eq;
use sea_orm::{ActiveModelTrait, DbErr, TryIntoModel};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use thiserror::Error;

use crate::created_at;
use crate::entities::account::{self, ActiveModel, Column, Entity, Model};
use crate::entities::ban;
use crate::entities::sea_orm_active_enums::GenderTy;

use super::password::PwService;
use super::DbConn;

pub type AccountId = i32;
pub type HardwareInfo = u32;

#[derive(Debug)]
#[repr(u8)]
pub enum Region {
    Asia = 1,
    America = 2,
    Europe = 3,
}

#[derive(Debug, Error)]
pub enum AccountServiceError {
    #[error("Account with the username already exists")]
    UsernameAlreadyExists,
    #[error("No account for this username was found")]
    UsernameNotFound,
    #[error("Password mismatch")]
    PasswordMismatch,
    #[error("Password size is wrong")]
    PasswordWrongSize,
    #[error("Password is only supposed to contain ASCII characters")]
    PasswordWrongChar,
    #[error("Password size is wrong")]
    UsernameWrongSize,
    #[error("Password is only supposed to contain ASCII characters")]
    UsernameWrongChar,
    #[error("Account is banned")]
    AccountBanned,
    #[error("Account already logged in")]
    AccountAlreadyLoggedIn,
    #[error("database")]
    Disconnect(#[from] DbErr),
}

//MAybe use passwords crate

pub type AccResult<T> = std::result::Result<T, AccountServiceError>;

#[derive(Debug)]
pub struct AccountService {
    db: DbConn,
    pw: PwService,
}

impl AccountService {
    pub fn new(db: DbConn) -> Self {
        Self {
            db,
            pw: PwService::default(),
        }
    }

    pub async fn get(&self, id: AccountId) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find_by_id(id).one(&self.db.0).await?)
    }

    pub async fn create(
        &self,
        username: impl ToString,
        password: &str,
        region: Region,
        accepted_tos: bool,
        gender: Option<GenderTy>,
    ) -> anyhow::Result<AccountId> {
        //TODO check username + password
        let hash = self.pw.generate_hash(password);

        let acc = ActiveModel {
            username: Set(username.to_string()),
            password_hash: Set(hash.to_string()),
            accepted_tos: Set(accepted_tos),
            created_at: created_at(&self.db.0),
            country: Set(region as u8 as i32),
            gm_level: Set(0),
            last_selected_world: Set(0),
            character_slots: Set(3),
            nx_credit: Set(0),
            shroom_points: Set(0),
            nx_prepaid: Set(0),
            gender: Set(gender),
            tester: Set(false),
            ..Default::default()
        };

        let res = Entity::insert(acc).exec(&self.db.0).await?;
        Ok(res.last_insert_id)
    }

    pub async fn try_login(&self, username: &str, password: &str) -> AccResult<Model> {
        let res = Entity::find()
            .filter(Column::Username.eq(username))
            //TODO find latest ban
            .find_also_related(ban::Entity)
            .one(&self.db.0)
            .await?;

        let Some((acc, last_ban)) = res else {
            return Err(AccountServiceError::UsernameNotFound);
        };

        if let Some(_last_ban) = last_ban {
            return Err(AccountServiceError::AccountBanned);
        }

        let verfiy_password = self.pw.verify_password(password, &acc.password_hash);
        if !verfiy_password {
            return Err(AccountServiceError::PasswordMismatch);
        }

        Ok(acc)
    }

    pub async fn update(
        &self,
        acc: Model,
        update: impl FnOnce(&mut ActiveModel),
    ) -> anyhow::Result<Model> {
        let mut acc: ActiveModel = acc.into();
        update(&mut acc);

        Ok(acc.save(&self.db.0).await?.try_into_model().unwrap())
    }

    pub async fn set_gender(&self, acc: Model, gender: GenderTy) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.gender = Set(Some(gender));
        })
        .await
    }

    pub async fn set_pic(&self, acc: Model, pic: String) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.pic = Set(Some(pic));
        })
        .await
    }

    pub async fn set_pin(&self, acc: Model, pin: String) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.pin = Set(Some(pin));
        })
        .await
    }

    pub async fn accept_tos(&self, acc: Model) -> anyhow::Result<Model> {
        self.update(acc, |acc| {
            acc.accepted_tos = Set(true);
        })
        .await
    }

    pub async fn delete_acc(&self, id: AccountId) -> anyhow::Result<()> {
        //TODO maybe do a soft delete
        account::Entity::delete_by_id(id).exec(&self.db.0).await?;
        Ok(())
    }

    pub fn check_pin(&self, acc: &Model, pin: &str) -> anyhow::Result<bool> {
        let Some(acc_pin) = acc.pin.as_ref() else {
            anyhow::bail!("Pin not set")
        };

        Ok(constant_time_eq(acc_pin.as_bytes(), pin.as_bytes()))
    }

    pub fn check_pic(&self, acc: &Model, pic: &str) -> anyhow::Result<bool> {
        let Some(acc_pic) = acc.pic.as_ref() else {
            anyhow::bail!("Pic not set")
        };

        Ok(constant_time_eq(acc_pic.as_bytes(), pic.as_bytes()))
    }

    pub fn check_hardware_info(
        &self,
        _acc: &Model,
        _hw_info: &HardwareInfo,
        _ip: IpAddr,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{entities::sea_orm_active_enums::GenderTy, services::account::Region};

    use super::AccountService;

    async fn get_test_svc() -> anyhow::Result<AccountService> {
        /*  let acc_svc = AccountService::new(get_test_db().await?);
        Ok(acc_svc)*/
        todo!()
    }

    #[tokio::test]
    async fn account_insert() -> anyhow::Result<()> {
        const USERNAME: &str = "test1";
        const PW: &str = "abc123";

        let svc = get_test_svc().await?;
        let acc_id = svc.create(USERNAME, PW, Region::Europe, true, None).await?;

        let acc = svc.get(acc_id).await?.unwrap();

        let acc = svc.set_gender(acc, GenderTy::Female).await?;
        assert_eq!(acc.gender, Some(GenderTy::Female));

        let acc = svc.accept_tos(acc).await?;
        assert!(acc.accepted_tos);

        //Login must work
        svc.try_login(USERNAME, PW).await?;

        Ok(())
    }
}
