pub mod char;
use chrono::{NaiveDateTime, TimeZone};
use shroom_pkt::ShroomTime;
use shroom_proto95::{
    login::{AccountGrade, AccountInfo, GradeCode, SubGradeCode},
    shared::Gender,
};

use crate::entities::{account, sea_orm_active_enums::GenderTy};

pub fn db_to_shroom_time(dt: NaiveDateTime) -> ShroomTime {
    ShroomTime::try_from(chrono::Utc.from_utc_datetime(&dt)).unwrap()
}

impl From<&GenderTy> for Gender {
    fn from(value: &GenderTy) -> Self {
        match value {
            GenderTy::Female => Self::Female,
            GenderTy::Male => Self::Male,
        }
    }
}

impl From<Gender> for GenderTy {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Female => Self::Female,
            Gender::Male => Self::Male,
        }
    }
}

impl From<&account::Model> for AccountInfo {
    fn from(model: &account::Model) -> Self {
        Self {
            id: model.id as u32,
            gender: model.gender.as_ref().into(),
            grade: AccountGrade {
                code: GradeCode::NORMAL,
                sub_code: SubGradeCode::NORMAL,
            },
            country_id: model.country as u8,
            name: model.username.clone(),
            purchase_exp: 0,
            chat_block_reason: 0,
            chat_block_date: ShroomTime::new(0),
            registration_date: db_to_shroom_time(model.created_at),
            num_chars: model.character_slots as u32,
        }
    }
}
