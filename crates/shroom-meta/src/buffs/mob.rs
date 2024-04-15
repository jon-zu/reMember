use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::id::{BuffId, CharacterId};

use super::{keys::MobBuffKey, BuffKey};

impl BuffKey for MobBuffKey {
    type Bits = u128;

    fn flag(&self) -> Self::Bits {
        1u128 << (*self as u8)
    }

    fn from_flag(flag: Self::Bits) -> Self {
        Self::try_from(flag.trailing_zeros() as u8).unwrap()
    }

    fn as_index(&self) -> usize {
        *self as usize
    }
}

pub trait MobBuffValue {
    fn to_buff_value(&self) -> i16;
}

impl MobBuffValue for i16 {
    fn to_buff_value(&self) -> i16 {
        *self
    }
}

pub trait MobBuffStat: Sized {
    const KEY: MobBuffKey;
    type Inner;

    fn get(storage: &MobBuffStorage) -> &MobBuff<Self>;
    fn get_mut(storage: &mut MobBuffStorage) -> &mut MobBuff<Self>;

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
    fn from_inner(inner: Self::Inner) -> Self;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MobBuff<T> {
    pub id: BuffId,
    pub data: T,
    pub dur: Duration,
    pub src: CharacterId,
}

impl<T> MobBuff<T> {
    pub fn new(id: BuffId, data: T, dur: Duration, src: CharacterId) -> Self {
        Self { id, data, dur, src }
    }
}

impl<T: MobBuffValue> MobBuffValue for MobBuff<T> {
    fn to_buff_value(&self) -> i16 {
        self.data.to_buff_value()
    }
}

macro_rules! mob_buff {
    ($buff:ident, $data:ty) => {
        #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $buff(pub $data);

        impl From<$data> for $buff {
            fn from(data: $data) -> Self {
                Self(data)
            }
        }

        impl From<$buff> for $data {
            fn from(buff: $buff) -> Self {
                buff.0
            }
        }

        impl MobBuffStat for $buff {
            const KEY: MobBuffKey = MobBuffKey::$buff;
            type Inner = $data;

            fn get(storage: &MobBuffStorage) -> &MobBuff<Self> {
                &storage.$buff
            }

            fn get_mut(storage: &mut MobBuffStorage) -> &mut MobBuff<Self> {
                &mut storage.$buff
            }

            fn inner(&self) -> &Self::Inner {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.0
            }

            fn from_inner(inner: Self::Inner) -> Self {
                Self(inner)
            }
        }

        paste::paste! {
            pub type [<MobBuff $buff>] = MobBuff<$buff>;
        }
    };
}

macro_rules! mob_buffs {
    ($($buff:ident,$ty:ty),*) => {
        $(
            mob_buff!($buff, $ty);
        )*

        #[derive(Debug, Default)]
        #[allow(non_snake_case)]
        pub struct MobBuffStorage {
            $(
                pub $buff: MobBuff<$buff>,
            )*
        }
    };
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct CounterData {
    pub n: i16,
    pub w: u32,
}

impl MobBuffValue for CounterData {
    fn to_buff_value(&self) -> i16 {
        self.n
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DisableData {
    pub invincible: bool,
    pub disable: bool,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct BurnData {
    pub n_dmg: u32,
    pub interval: Duration,
    pub dot_count: u32,
}

mob_buffs!(
    Pad,
    i16,
    Pdr,
    i16,
    Mad,
    i16,
    Mdr,
    i16,
    Acc,
    i16,
    Eva,
    i16,
    Speed,
    i16,
    Stun,
    i16,
    Freeze,
    i16,
    Poison,
    i16,
    Seal,
    i16,
    Darkness,
    i16,
    PowerUp,
    i16,
    MagicUp,
    i16,
    PGuardUp,
    i16,
    MGuardUp,
    i16,
    Doom,
    i16,
    Web,
    i16,
    PImmune,
    i16,
    MImmune,
    i16,
    HardSkin,
    i16,
    Ambush,
    i16,
    Venom,
    i16,
    Blind,
    i16,
    SealSkill,
    i16,
    Dazzle,
    i16,
    PCounter,
    CounterData,
    MCounter,
    CounterData,
    RiseByToss,
    i16,
    BodyPressure,
    i16,
    Weakness,
    i16,
    TimeBomb,
    i16,
    Showdown,
    i16,
    MagicCrash,
    i16,
    DamagedElemAttr,
    i16,
    HealByDamage,
    i16,
    Burned,
    BurnData,
    Disable,
    DisableData
);

impl MobBuffStorage {
    pub fn set<T: MobBuffStat>(&mut self, buff: MobBuff<T>) {
        *T::get_mut(self) = buff;
    }

    pub fn get<T: MobBuffStat>(&self) -> &MobBuff<T> {
        T::get(self)
    }

    pub fn get_mut<T: MobBuffStat>(&mut self) -> &mut MobBuff<T> {
        T::get_mut(self)
    }

    pub fn update<T: MobBuffStat>(&mut self, f: impl Fn(&mut MobBuff<T>)) {
        f(self.get_mut::<T>());
    }
}

/*

pub struct MobBuffPacket<'a> {
    pub storage: &'a MobBuffStorage,
    pub flags: MobTemporaryStatFlags,
}

impl<'a> EncodePacket for MobBuffPacket<'a> {
    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::NONE;

    fn encode<B: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<B>,
    ) -> shroom_pkt::PacketResult<()> {
        let f = self.flags.clone();
        f.encode(pw)?;

        macro_rules! encode_buff {
            ($buff:ident) => {
                if f.contains(MobTemporaryStatFlags::$buff) {
                    TempStatValue {
                        value: self.storage.$buff.data.0,
                        dur: self.storage.$buff.dur.into(),
                        reason: self.storage.$buff.id,
                    }
                    .encode(pw)?;
                }
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
            let b = &self.storage.PCounter;
            TempStatValue {
                value: b.data.0.n,
                dur: b.dur.into(),
                reason: b.id,
            }
            .encode(pw)?;
        }
        let mcounter = f.contains(MobTemporaryStatFlags::PCounter);
        if mcounter {
            let b = &self.storage.PCounter;
            TempStatValue {
                value: b.data.0.n,
                dur: b.dur.into(),
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
            let b = &self.storage.Burned;
            // Dummy list
            (1u32).encode(pw)?;
            BurnedInfo {
                char_id: b.src,
                skill_id: SkillId(b.id),
                n_dmg: b.data.0.n_dmg,
                interval: b.data.0.interval.into(),
                end: b.dur.into(),
                dot_count: b.data.0.dot_count,
            }
            .encode(pw)?;
        }

        if pcounter {
            self.storage.PCounter.data.0.w.encode(pw)?;
        }
        if mcounter {
            self.storage.MCounter.data.0.w.encode(pw)?;
        }
        if mcounter || pcounter {
            // todo: Counter probability
            (100u32).encode(pw)?;
        }
        if f.contains(MobTemporaryStatFlags::Disable) {
            let b = &self.storage.Disable;
            b.data.0.invincible.encode(pw)?;
            b.data.0.disable.encode(pw)?;
        }
        //TODO: stat changed for movement affected
        (0u8).encode(pw)?;

        Ok(())
    }
}*/

#[cfg(test)]
mod tests {
    use crate::id::SkillId;

    use super::*;

    #[test]
    fn mob_buff_set_get() {
        let mut storage = MobBuffStorage::default();
        storage.set(MobBuffPad::new(
            BuffId::Skill(SkillId(1337)),
            Pad(10),
            Duration::from_secs(10),
            0.into(),
        ));
        assert_eq!(storage.get::<Pad>().id, BuffId::Skill(SkillId(1337)));
        assert_eq!(storage.get::<Pad>().data.0, 10);
        storage.update::<Pad>(|b| b.data.0 += 1);
        assert_eq!(storage.get::<Pad>().data.0, 11);
    }
}
