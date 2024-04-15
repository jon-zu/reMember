use std::time::Duration;

use crate::id::{BuffId, ItemId, ObjectId};
use crate::{
    shared::ElementAttribute,
    skill::{Skill, SkillLevel},
};

use super::keys::CharBuffKey;
use super::{BuffKey, SkillChance};

impl BuffKey for CharBuffKey {
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

pub trait CharBuffValue {
    fn to_buff_value(&self) -> i16;
}

impl CharBuffValue for i16 {
    fn to_buff_value(&self) -> i16 {
        *self
    }
}

pub trait CharBuffStat: Sized {
    const KEY: CharBuffKey;
    type Inner;

    fn get(storage: &CharBuffStorage) -> &CharBuff<Self>;
    fn get_mut(storage: &mut CharBuffStorage) -> &mut CharBuff<Self>;

    fn from_inner(inner: Self::Inner) -> Self;

    fn extend_duration() -> bool {
        Self::KEY == CharBuffKey::EnergyCharged
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharBuff<T> {
    pub id: BuffId,
    pub data: T,
    pub dur: Duration,
}

impl<T> CharBuff<T> {
    pub fn new(id: BuffId, data: T, dur: Duration) -> Self {
        Self { id, data, dur }
    }

    pub fn from_skill<U: Into<T>>(skill: &Skill, lvl: SkillLevel, data: U) -> Self {
        Self {
            id: skill.id.into(),
            data: data.into(),
            dur: skill.time_dur(lvl),
        }
    }

    pub fn from_skill_x(skill: &Skill, lvl: SkillLevel) -> Self
    where
        T: From<i16>,
    {
        Self {
            id: skill.id.into(),
            data: skill.x(lvl).into(),
            dur: skill.time_dur(lvl),
        }
    }

    pub fn from_skill_y(skill: &Skill, lvl: SkillLevel) -> Self
    where
        T: From<i16>,
    {
        Self {
            id: skill.id.into(),
            data: skill.y(lvl).into(),
            dur: skill.time_dur(lvl),
        }
    }

    pub fn from_skill_z(skill: &Skill, lvl: SkillLevel) -> Self
    where
        T: From<i16>,
    {
        Self {
            id: skill.id.into(),
            data: skill.y(lvl).into(),
            dur: skill.time_dur(lvl),
        }
    }
}

macro_rules! char_buff {
    ($buff:ident, $data:ty, new) => {
        #[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
        pub struct $buff(pub $data);

        impl From<$data> for $buff {
            fn from(data: $data) -> Self {
                Self(data)
            }
        }

        impl CharBuffValue for $buff {
            fn to_buff_value(&self) -> i16 {
                self.0.to_buff_value()
            }
        }

        char_buff!($buff, $buff,);
    };
    ($buff:ident, $data:ty,) => {
        impl CharBuffStat for $data {
            const KEY: CharBuffKey = CharBuffKey::$buff;
            type Inner = $data;

            fn get(storage: &CharBuffStorage) -> &CharBuff<Self> {
                &storage.$buff
            }

            fn get_mut(storage: &mut CharBuffStorage) -> &mut CharBuff<Self> {
                &mut storage.$buff
            }

            fn from_inner(inner: Self::Inner) -> Self {
                Self::from(inner)
            }
        }

        paste::paste! {
            pub type [<CharBuff $buff>] = CharBuff<$data>;
        }
    };
}

//TODO right now the data type has to be same name as the flag
macro_rules! char_buffs {
    ($($buff:ident($ty:ty$(=$new:tt)?)),*)  => {
        $(
            char_buff!($buff, $ty, $($new)?);
        )*

        #[derive(Debug, Default)]
        #[allow(non_snake_case)]
        pub struct CharBuffStorage {
            $(
                pub $buff: CharBuff<$buff>,
            )*
        }
    };
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct Dice {
    pub value: i16,
    pub stats: [u32; 0x16],
}

impl CharBuffValue for Dice {
    fn to_buff_value(&self) -> i16 {
        self.value
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct BlessingArmor {
    pub value: i16,
    pub pad: u32,
}

impl CharBuffValue for BlessingArmor {
    fn to_buff_value(&self) -> i16 {
        self.value
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct ComboCounter {
    pub orbs: i16,
    pub max_orbs: i16,
    pub double_proc_rate: SkillChance,
    pub damage_per_orb: i16,
}

impl ComboCounter {
    pub fn proc(&mut self) -> bool {
        if self.double_proc_rate.proc() {
            let res = self.increment();
            self.increment();
            res
        } else {
            self.increment()
        }
    }

    pub fn increment(&mut self) -> bool {
        if self.orbs <= self.max_orbs {
            self.orbs += 1;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) -> bool {
        if self.orbs != 1 {
            self.orbs = 1;
            true
        } else {
            false
        }
    }
}

impl CharBuffValue for ComboCounter {
    fn to_buff_value(&self) -> i16 {
        self.orbs
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct SharpEyes {
    pub crit_rate: u8,
    pub crit_dmg_max: u8,
}

impl CharBuffValue for SharpEyes {
    fn to_buff_value(&self) -> i16 {
        (self.crit_rate as i16) << 8 | self.crit_dmg_max as i16
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct DragonBlood {
    pub dec_hp: i16,
    pub pad: i16,
    pub tick_interval: Duration,
}

impl CharBuffValue for DragonBlood {
    fn to_buff_value(&self) -> i16 {
        1
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeaponCharge {
    pub elem: ElementAttribute,
    pub damage: i16,
    // Freeze duration for ice
    pub elem_apply: i16,
}

impl CharBuffValue for WeaponCharge {
    fn to_buff_value(&self) -> i16 {
        1
    }
}

impl Default for WeaponCharge {
    fn default() -> Self {
        Self {
            elem: ElementAttribute::Dark,
            damage: 0,
            elem_apply: 0,
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct SpiritJavelin(pub ItemId);

impl From<ItemId> for SpiritJavelin {
    fn from(item_id: ItemId) -> Self {
        Self(item_id)
    }
}

impl CharBuffValue for SpiritJavelin {
    fn to_buff_value(&self) -> i16 {
        ((self.0 .0 - ItemId::THROWING_STAR_MIN.0) + 1) as i16
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct SwallowAttackDamage {
    pub atk_damage: i16,
    pub critical: i16,
    pub max_hp: i16,
    pub defense: i16,
    pub evasion: i16,
    pub time: i16,
}

impl CharBuffValue for SwallowAttackDamage {
    fn to_buff_value(&self) -> i16 {
        self.atk_damage
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct RideVehicle(pub u32);

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct GuidedBullet {
    pub mob_id: ObjectId,
    pub value: u32,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct PartyBooster(pub i32);

impl From<i16> for PartyBooster {
    fn from(data: i16) -> Self {
        Self(data as i32)
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct EnergyCharged {
    pub energy: u32,
    pub max_energy: u32,
    pub energy_per_attack: u32,
}

impl EnergyCharged {
    pub fn proc(&mut self) -> bool {
        if self.energy < self.max_energy {
            self.energy += self.energy_per_attack;
            self.energy = self.energy.min(self.max_energy);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) -> bool {
        if self.energy != 0 {
            self.energy = 0;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct DefenseAtt {
    pub value: i16,
    pub attr: u8,
}

impl CharBuffValue for DefenseAtt {
    fn to_buff_value(&self) -> i16 {
        self.value
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct DefenseState {
    pub value: i16,
    pub attr: u8,
}

impl CharBuffValue for DefenseState {
    fn to_buff_value(&self) -> i16 {
        self.value
    }
}

char_buffs!(
    Pad(i16 = new),
    Pdd(i16 = new),
    Mad(i16 = new),
    Mdd(i16 = new),
    Acc(i16 = new),
    Evasion(i16 = new),
    CriticalRate(i16 = new),
    Speed(i16 = new),
    Jump(i16 = new),
    ExtraMaxHp(i16 = new),
    ExtraMaxMp(i16 = new),
    ExtraPad(i16 = new),
    ExtraPdd(i16 = new),
    ExtraMdd(i16 = new),
    MagicGuard(i16 = new),
    DarkSight(i16 = new),
    Booster(i16 = new),
    PowerGuard(i16 = new),
    Guard(i16 = new),
    SafetyDamage(i16 = new),
    SafetyAbsorb(i16 = new),
    MaxHp(i16 = new),
    MaxMp(i16 = new),
    Invincible(i16 = new),
    SoulArrow(i16 = new),
    Stun(i16 = new),
    Poison(i16 = new),
    Seal(i16 = new),
    Darkness(i16 = new),
    ComboCounter(ComboCounter),
    WeaponCharge(WeaponCharge),
    DragonBlood(DragonBlood),
    HolySymbol(i16 = new),
    MesoUp(i16 = new),
    ShadowPartner(i16 = new),
    PickPocket(i16 = new),
    MesoGuard(i16 = new),
    Thaw(i16 = new),
    Weakness(i16 = new),
    Curse(i16 = new),
    Slow(i16 = new),
    Morph(i16 = new),
    Ghost(i16 = new),
    Regen(i16 = new),
    BasicStatUp(i16 = new),
    Stance(i16 = new),
    SharpEyes(SharpEyes),
    ManaReflection(i16 = new),
    Attract(i16 = new),
    SpiritJavelin(SpiritJavelin),
    Infinity(i16 = new),
    Holyshield(i16 = new),
    HamString(i16 = new),
    Blind(i16 = new),
    Concentration(i16 = new),
    BanMap(i16 = new),
    MaxLevelBuff(i16 = new),
    Barrier(i16 = new),
    DojangShield(i16 = new),
    ReverseInput(i16 = new),
    MesoUpByItem(i16 = new),
    ItemUpByItem(i16 = new),
    RespectPImmune(i16 = new),
    RespectMImmune(i16 = new),
    DefenseAtt(DefenseAtt),
    DefenseState(DefenseState),
    DojangBerserk(i16 = new),
    DojangInvincible(i16 = new),
    Spark(i16 = new),
    SoulMasterFinal(i16 = new),
    WindBreakerFinal(i16 = new),
    ElementalReset(i16 = new),
    WindWalk(i16 = new),
    EventRate(i16 = new),
    ComboAbilityBuff(i16 = new),
    ComboDrain(i16 = new),
    ComboBarrier(i16 = new),
    BodyPressure(i16 = new),
    SmartKnockback(i16 = new),
    RepeatEffect(i16 = new),
    ExpBuffRate(i16 = new),
    IncEffectHPPotion(i16 = new),
    IncEffectMPPotion(i16 = new),
    StopPortion(i16 = new),
    StopMotion(i16 = new),
    Fear(i16 = new),
    EvanSlow(i16 = new),
    MagicShield(i16 = new),
    MagicResistance(i16 = new),
    SoulStone(i16 = new),
    Flying(i16 = new),
    Frozen(i16 = new),
    AssistCharge(i16 = new),
    Enrage(i16 = new),
    SuddenDeath(i16 = new),
    NotDamaged(i16 = new),
    FinalCut(i16 = new),
    ThornsEffect(i16 = new),
    SwallowAttackDamage(SwallowAttackDamage),
    MorewildDamageUp(i16 = new),
    Mine(i16 = new),
    Cyclone(i16 = new),
    SwallowCritical(i16 = new),
    SwallowMaxMP(i16 = new),
    SwallowDefence(i16 = new),
    SwallowEvasion(i16 = new),
    Conversion(i16 = new),
    Revive(i16 = new),
    Sneak(i16 = new),
    Mechanic(i16 = new),
    Aura(i16 = new),
    DarkAura(i16 = new),
    BlueAura(i16 = new),
    YellowAura(i16 = new),
    SuperBody(i16 = new),
    MorewildMaxHP(i16 = new),
    Dice(Dice),
    BlessingArmor(BlessingArmor),
    DamR(i16 = new),
    TeleportMasteryOn(i16 = new),
    CombatOrders(i16 = new),
    Beholder(i16 = new),
    EnergyCharged(EnergyCharged),
    DashSpeed(i16 = new),
    DashJump(i16 = new),
    RideVehicle(RideVehicle),
    PartyBooster(PartyBooster),
    GuidedBullet(GuidedBullet)
);

impl CharBuffStorage {
    pub fn set<T: CharBuffStat>(&mut self, buff: CharBuff<T>) {
        *T::get_mut(self) = buff;
    }

    pub fn get<T: CharBuffStat>(&self) -> &CharBuff<T> {
        T::get(self)
    }

    pub fn get_mut<T: CharBuffStat>(&mut self) -> &mut CharBuff<T> {
        T::get_mut(self)
    }

    pub fn update<T: CharBuffStat>(&mut self, f: impl Fn(&mut CharBuff<T>)) {
        f(self.get_mut::<T>());
    }
}

#[cfg(test)]
mod tests {
    use crate::id::SkillId;

    use super::*;

    #[test]
    fn char_buff_set_get() {
        let mut storage = CharBuffStorage::default();
        storage.set(CharBuffPad::new(SkillId(1337).into(), Pad(10), Duration::from_secs(10)));
        assert_eq!(storage.get::<Pad>().id.to_src32(), 1337);
        assert_eq!(storage.get::<Pad>().data.0, 10);
        storage.update::<Pad>(|b| b.data.0 += 1);
        assert_eq!(storage.get::<Pad>().data.0, 11);
    }
}
