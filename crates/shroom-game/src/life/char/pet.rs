use anyhow::Context;
use either::Either;
use shroom_meta::{
    id::{CashID, CharacterId, FootholdId},
    twod::Vec2,
};
use shroom_proto95::game::user::pet::{
    PetInitData, PetLocalActivateResp, PetLocalActivateResult, PetRemoteActivateResp,
    PetRemoteEnterFieldResp,
};

use super::Character;

const PET_LIMIT: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pet {
    char_id: CharacterId,
    id: usize,
    pos: Vec2,
    fh: FootholdId,
    intial: bool,
    pub tmpl_id: u32,
    pub name: String,
    pub name_tag: bool,
    pub chat_balloon: bool,
    pub sn: CashID,
}

impl Pet {
    pub fn new(tmpl_id: u32, name: String, cash_id: CashID) -> Self {
        Self {
            char_id: CharacterId::default(),
            id: 0,
            pos: Vec2::default(),
            fh: FootholdId::default(),
            tmpl_id,
            name,
            name_tag: false,
            chat_balloon: false,
            sn: cash_id,
            intial: true,
        }
    }

    pub fn assign_char(&mut self, chr: &mut Character) {
        self.char_id = chr.id;
        self.pos = chr.pos;
        self.fh = chr.fh;
    }

    pub fn pet_data(&self) -> PetInitData {
        PetInitData {
            reset_active: false,
            pet_tmpl_id: self.tmpl_id,
            pet_name: self.name.clone(),
            pet_locker_sn: self.sn,
            pos: self.pos,
            move_action: 0,
            fh: self.fh,
            name_tag: self.name_tag,
            chat_balloon: self.chat_balloon,
        }
    }

    pub fn reset_data(&self) -> PetInitData {
        PetInitData {
            reset_active: true,
            pet_tmpl_id: 0,
            pet_name: String::new(),
            pet_locker_sn: 0,
            pos: Vec2::default(),
            move_action: 0,
            fh: FootholdId::default(),
            name_tag: false,
            chat_balloon: false,
        }
    }

    pub fn local_enter_msg(&self) -> PetLocalActivateResp {
        PetLocalActivateResp {
            pet_id: self.id as u8,
            char: self.char_id,
            pet_data: PetLocalActivateResult::Ok(self.pet_data()),
        }
    }

    pub fn local_leave_msg(&self) -> PetLocalActivateResp {
        PetLocalActivateResp {
            pet_id: self.id as u8,
            char: self.char_id,
            pet_data: PetLocalActivateResult::Ok(self.reset_data()),
        }
    }

    pub fn enter_msg(&self, first: bool) -> Either<PetRemoteActivateResp, PetRemoteEnterFieldResp> {
        let pet_data = self.pet_data();
        if !first {
            Either::Right(PetRemoteEnterFieldResp {
                char: self.char_id,
                pet_id: self.id as u8,
                pet_data: Some(pet_data).into(),
            })
        } else {
            Either::Left(PetRemoteActivateResp {
                char: self.char_id,
                pet_id: self.id as u8,
                pet_data: Some(pet_data).into(),
            })
        }
    }

    pub fn leave_msg(&self, _param: ()) -> PetRemoteEnterFieldResp {
        PetRemoteEnterFieldResp {
            char: self.char_id,
            pet_id: self.id as u8,
            pet_data: Some(self.reset_data()).into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CharPets(pub [Option<Pet>; PET_LIMIT]);

impl CharPets {
    pub fn get(&self, ix: usize) -> Option<&Pet> {
        self.0.get(ix).and_then(|x| x.as_ref())
    }

    pub fn get_mut(&mut self, ix: usize) -> Option<&mut Pet> {
        self.0.get_mut(ix).and_then(|x| x.as_mut())
    }

    pub fn free_slots(&self) -> usize {
        self.0.iter().filter(|x| x.is_none()).count()
    }

    pub fn add_pet(&mut self, mut pet: Pet) -> anyhow::Result<usize> {
        let free_ix = self
            .0
            .iter()
            .position(|x| x.is_none())
            .context("No free slot")?;
        pet.id = free_ix;
        self.0[free_ix] = Some(pet);
        Ok(free_ix)
    }
}
