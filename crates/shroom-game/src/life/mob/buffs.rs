use std::time::Duration;

use bytes::BufMut;
use shroom_meta::{
    buffs::{
        mob::{
            self, Burned, MCounter, MImmune, MobBuff, MobBuffStat, MobBuffStorage, MobBuffValue,
            PCounter, PImmune,
        },
        BuffKey,
    },
    class::{warrior::ThreatenAttack, MobDebuff},
    id::{BuffId, CharacterId},
    mob::{MobBuffSkill, MobBuffSkillData},
};

use shroom_pkt::PacketResult;
use shroom_pkt::{EncodePacket, PacketWriter};
use shroom_proto95::game::life::mob::{BurnedInfo, MobTemporaryStatFlags, TempStatValue};
use shroom_srv::GameTime;

use crate::life::char::buffs::BuffExpirations;

//TODO burning needs a complete revamp due to stacking buff logic

pub trait MobApplyDebuff {
    fn apply_debuff(&self, buffs: &mut MobBuffs, t: GameTime, id: BuffId, src: CharacterId);
}

impl<T: MobBuffStat + Clone> MobApplyDebuff for MobDebuff<T> {
    fn apply_debuff(&self, buffs: &mut MobBuffs, t: GameTime, id: BuffId, src: CharacterId) {
        if self.proc.proc() {
            buffs.set(t, MobBuff::new(id, self.stat.clone(), self.dur, src));
        }
    }
}

impl MobApplyDebuff for ThreatenAttack {
    fn apply_debuff(&self, buffs: &mut MobBuffs, t: GameTime, id: BuffId, src: CharacterId) {
        self.atk_debuff.apply_debuff(buffs, t, id, src);
        self.def_debuff.apply_debuff(buffs, t, id, src);
        self.acc_debuff.apply_debuff(buffs, t, id, src);
    }
}

impl<T: MobBuffStat + Clone> MobApplyDebuff for MobBuffSkill<T> {
    fn apply_debuff(&self, buffs: &mut MobBuffs, t: GameTime, id: BuffId, src: CharacterId) {
        //TODO do extension
        buffs.set(
            t,
            MobBuff::new(id, self.stat.data.clone(), self.stat.dur, src),
        )
    }
}

impl MobApplyDebuff for MobBuffSkillData {
    fn apply_debuff(&self, buffs: &mut MobBuffs, t: GameTime, id: BuffId, src: CharacterId) {
        match self {
            MobBuffSkillData::PowerUp(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::MagicUp(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::PGuardUp(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::MGuardUp(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Haste(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Speed(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::PhysicalImmune(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::MagicImmune(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::HardSkin(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Pad(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Mad(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Pdr(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Mdr(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Acc(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::Eva(d) => d.apply_debuff(buffs, t, id, src),
            MobBuffSkillData::PhyicalCounter(d) => {
                buffs.set(
                    t,
                    MobBuff::new(d.stat.id, mob::PImmune(1), d.stat.dur, d.stat.src),
                );
                d.apply_debuff(buffs, t, id, src)
            }
            MobBuffSkillData::MagicCounter(d) => {
                buffs.set(
                    t,
                    MobBuff::new(d.stat.id, mob::MImmune(1), d.stat.dur, d.stat.src),
                );
                d.apply_debuff(buffs, t, id, src)
            }
            MobBuffSkillData::PMCounter(p, m) => {
                buffs.set(
                    t,
                    MobBuff::new(p.stat.id, mob::PImmune(1), p.stat.dur, p.stat.src),
                );
                p.apply_debuff(buffs, t, id, src);
                buffs.set(
                    t,
                    MobBuff::new(m.stat.id, mob::MImmune(1), m.stat.dur, m.stat.src),
                );
                m.apply_debuff(buffs, t, id, src);
            }
        }
    }
}

pub struct MobRemovedBuffs {
    pub flags: MobTemporaryStatFlags,
    pub removed_burn: Option<(CharacterId, BuffId)>,
}

#[derive(Debug)]
pub struct MobBuffs {
    buff_flag: MobTemporaryStatFlags,
    remove_flag: MobTemporaryStatFlags,
    update_flag: MobTemporaryStatFlags,
    storage: MobBuffStorage,
    exp: BuffExpirations<64, shroom_meta::buffs::keys::MobBuffKey>,
}

impl Default for MobBuffs {
    fn default() -> Self {
        Self {
            buff_flag: MobTemporaryStatFlags::empty(),
            remove_flag: MobTemporaryStatFlags::empty(),
            update_flag: MobTemporaryStatFlags::empty(),
            storage: MobBuffStorage::default(),
            exp: BuffExpirations::default(),
        }
    }
}

impl MobBuffs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_immunes(&mut self) {
        self.remove::<PImmune>();
        self.remove::<MImmune>();
    }

    pub fn set_if_not_exists<T: MobBuffStat>(&mut self, t: GameTime, buff: MobBuff<T>) {
        if self.get::<T>().is_none() {
            self.set(t, buff);
        }
    }

    pub fn set<T: MobBuffStat>(&mut self, t: GameTime, buff: MobBuff<T>) {
        // TODO, hacky workaround for burn
        // only allow one burn at a time for now
        if T::KEY == shroom_meta::buffs::keys::MobBuffKey::Burned
            && self.buff_flag.contains(MobTemporaryStatFlags::Burned)
        {
            log::info!("Mob burn already exists");
            return;
        }

        let f = MobTemporaryStatFlags::from_bits_truncate(T::KEY.flag());
        self.buff_flag.insert(f.clone());
        self.update_flag.insert(f.clone());
        self.remove_flag.remove(f);
        self.exp.insert(T::KEY, t + buff.dur);
        self.storage.set(buff);
    }

    pub fn get<T: MobBuffStat>(&self) -> Option<&MobBuff<T>> {
        let f = MobTemporaryStatFlags::from_bits_truncate(T::KEY.flag());
        if self.buff_flag.contains(f) {
            Some(T::get(&self.storage))
        } else {
            None
        }
    }

    pub fn remove<T: MobBuffStat>(&mut self) {
        let f = MobTemporaryStatFlags::from_bits_truncate(T::KEY.flag());
        self.buff_flag.remove(f.clone());
        self.remove_flag.insert(f);
        self.exp.mark_removed(T::KEY);
    }

    pub fn update<T: MobBuffStat>(&mut self, f: impl Fn(&mut T) -> bool) {
        let fl = MobTemporaryStatFlags::from_bits_truncate(T::KEY.flag());
        if self.buff_flag.contains(fl.clone()) {
            let buff = T::get_mut(&mut self.storage);
            if f(&mut buff.data) {
                self.update_flag.insert(fl);
            }
        }
    }

    pub fn update_expirations(&mut self, t: GameTime) -> Option<MobRemovedBuffs> {
        while let Some(exp) = self.exp.next_expired(t) {
            let f = MobTemporaryStatFlags::from_bits_truncate(exp.flag());
            self.buff_flag.remove(f.clone());
            self.remove_flag.insert(f);
        }

        if self.remove_flag.is_empty() {
            return None;
        }

        let burn_remove = if self.remove_flag.contains(MobTemporaryStatFlags::Burned) {
            let b = &self.storage.Burned;
            Some((b.src, b.id))
        } else {
            None
        };

        Some(MobRemovedBuffs {
            flags: std::mem::take(&mut self.remove_flag),
            removed_burn: burn_remove,
        })
    }

    pub fn take_updated(&mut self) -> MobTemporaryStatFlags {
        let f = std::mem::take(&mut self.update_flag);
        f.intersection(self.buff_flag.clone())
    }

    #[inline]
    fn encode_if_flag<T: MobBuffStat, B: BufMut>(
        &self,
        pw: &mut PacketWriter<B>,
        t: GameTime,
    ) -> PacketResult<()>
    where
        T::Inner: MobBuffValue,
    {
        if let Some(b) = self.get::<T>() {
            TempStatValue {
                value: b.data.inner().to_buff_value(),
                reason: b.id,
                dur: self.exp.expiration_dur(T::KEY, t).into(),
            }
            .encode(pw)?;
        }

        Ok(())
    }

    fn exp_dur<T: MobBuffStat>(&self, t: GameTime) -> Duration {
        self.exp.expiration_dur(T::KEY, t)
    }
}

pub struct MobBuffPacket<'a> {
    pub buffs: &'a MobBuffs,
    pub flags: MobTemporaryStatFlags,
    pub t: GameTime,
}

impl<'a> EncodePacket for MobBuffPacket<'a> {
    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::NONE;

    fn encode<B: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<B>,
    ) -> shroom_pkt::PacketResult<()> {
        let st = &self.buffs.storage;
        let f = self.flags.clone();
        f.encode(pw)?;

        macro_rules! encode_buff {
            ($buff:ident) => {
                self.buffs
                    .encode_if_flag::<shroom_meta::buffs::mob::$buff, _>(pw, self.t)?;
            };
        }

        encode_buff!(Pad);
        encode_buff!(Pdr);
        encode_buff!(Mad);
        encode_buff!(Mdr);
        encode_buff!(Acc);
        encode_buff!(Eva);
        encode_buff!(Speed);
        encode_buff!(Stun);
        encode_buff!(Freeze);
        encode_buff!(Poison);
        encode_buff!(Seal);
        encode_buff!(Darkness);
        encode_buff!(PowerUp);
        encode_buff!(MagicUp);
        encode_buff!(PGuardUp);
        encode_buff!(MGuardUp);
        encode_buff!(Doom);
        encode_buff!(Web);
        encode_buff!(PImmune);
        encode_buff!(MImmune);
        encode_buff!(HardSkin);
        encode_buff!(Ambush);
        encode_buff!(Venom);
        encode_buff!(Blind);
        encode_buff!(SealSkill);
        encode_buff!(Dazzle);
        let pcounter = f.contains(MobTemporaryStatFlags::PCounter);
        if pcounter {
            let b = &st.PCounter;
            TempStatValue {
                value: b.data.0.n,
                dur: self.buffs.exp_dur::<PCounter>(self.t).into(),
                reason: b.id,
            }
            .encode(pw)?;
        }
        let mcounter = f.contains(MobTemporaryStatFlags::MCounter);
        if mcounter {
            let b = &st.MCounter;
            TempStatValue {
                value: b.data.0.n,
                dur: self.buffs.exp_dur::<MCounter>(self.t).into(),
                reason: b.id,
            }
            .encode(pw)?;
        }
        encode_buff!(RiseByToss);
        encode_buff!(BodyPressure);
        encode_buff!(Weakness);
        encode_buff!(TimeBomb);
        encode_buff!(Showdown);
        encode_buff!(MagicCrash);
        encode_buff!(DamagedElemAttr);
        encode_buff!(HealByDamage);
        if f.contains(MobTemporaryStatFlags::Burned) {
            let b = &st.Burned;
            // Dummy list
            (1u32).encode(pw)?;
            BurnedInfo {
                char_id: b.src,
                skill_id: b.id,
                n_dmg: b.data.0.n_dmg,
                interval: b.data.0.interval.into(),
                end: self.buffs.exp_dur::<Burned>(self.t).into(),
                dot_count: b.data.0.dot_count,
            }
            .encode(pw)?;
        }

        if pcounter {
            st.PCounter.data.0.w.encode(pw)?;
        }
        if mcounter {
            st.MCounter.data.0.w.encode(pw)?;
        }
        if mcounter || pcounter {
            // todo: Counter probability
            (100u32).encode(pw)?;
        }
        if f.contains(MobTemporaryStatFlags::Disable) {
            let b = &st.Disable;
            b.data.0.invincible.encode(pw)?;
            b.data.0.disable.encode(pw)?;
        }
        //TODO: use the keys proprely
        let movement_affecting = f.intersects(
            MobTemporaryStatFlags::Speed
                | MobTemporaryStatFlags::Stun
                | MobTemporaryStatFlags::Freeze
                | MobTemporaryStatFlags::Doom
                | MobTemporaryStatFlags::RiseByToss,
        );
        movement_affecting.encode(pw)?;

        Ok(())
    }
}
