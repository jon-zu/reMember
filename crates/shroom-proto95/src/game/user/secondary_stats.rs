use std::time::Duration;

use crate::send_opcodes::SendOpcodes;
use shroom_pkt::{
    partial::{PartialData, PartialFlag},
    partial_data,
    time::{ClientTimeOffset, DurationMs},
    with_opcode, CondOption, HasOpCode, OptionTail, PacketConditional, ShroomDurationMs16,
    ShroomDurationMs32, ShroomPacket,
};
use shroom_meta::id::MobId;

#[derive(ShroomPacket, Debug, Clone)]
pub struct TempStatValue {
    /// Value of the buff
    pub value: i16,
    /// Reason for the buff, usually the buff id
    pub reason: u32,
    /// Buff duration
    pub duration: ShroomDurationMs32,
}

impl TempStatValue {
    pub fn new(value: i16, reason: u32, duration: Duration) -> Self {
        Self {
            value,
            reason,
            duration: ShroomDurationMs32::from(duration),
        }
    }
}

impl Default for TempStatValue {
    fn default() -> Self {
        Self {
            value: 0,
            reason: 0,
            duration: DurationMs(0),
        }
    }
}

partial_data!(
    RemoteCharSecondaryStat,
    RemoteCharSecondaryStatFlags,
    u128,
    derive(Debug, Clone),
    // n
    Speed(u8) => CharSecondaryStatFlags::Speed.bits(),
    // n
    ComboCounter(u8) => CharSecondaryStatFlags::ComboCounter.bits(),
    // r
    WeaponCharge(u32) => CharSecondaryStatFlags::WeaponCharge.bits(),
    // r
    Stun(u32) => CharSecondaryStatFlags::Stun.bits(),
    // r
    Darkness(u32) => CharSecondaryStatFlags::Darkness.bits(),
    // r
    Seal(u32) => CharSecondaryStatFlags::Seal.bits(),
    // r
    Weakness(u32) => CharSecondaryStatFlags::Weakness.bits(),
    // r
    Curse(u32) => CharSecondaryStatFlags::Curse.bits(),
    // n, r
    Poison((u16, u32)) => CharSecondaryStatFlags::Poison.bits(),
    // r
    ShadowPartner(u32) => CharSecondaryStatFlags::ShadowPartner.bits(),
    DarkSight(()) => CharSecondaryStatFlags::DarkSight.bits(),
    SoulArrow(()) => CharSecondaryStatFlags::SoulArrow.bits(),
    // n
    Morph(u16) => CharSecondaryStatFlags::Morph.bits(),
    // n
    Ghost(u16) => CharSecondaryStatFlags::Ghost.bits(),
    // r
    Attract(u32) => CharSecondaryStatFlags::Attract.bits(),
    // n
    SpiritJavelin(u32) => CharSecondaryStatFlags::SpiritJavelin.bits(),
    // r
    BanMap(u32) => CharSecondaryStatFlags::BanMap.bits(),
    // r
    Barrier(u32) => CharSecondaryStatFlags::Barrier.bits(),
    // r
    DojangShield(u32) => CharSecondaryStatFlags::DojangShield.bits(),
    // r
    ReverseInput(u32) => CharSecondaryStatFlags::ReverseInput.bits(),
    // n
    RespectPImmune(u32) => CharSecondaryStatFlags::RespectPImmune.bits(),
    // n
    RespectMImmune(u32) => CharSecondaryStatFlags::RespectMImmune.bits(),
    // n
    DefenseAtt(u32) => CharSecondaryStatFlags::DefenseAtt.bits(),
    // n
    DefenseState(u32) => CharSecondaryStatFlags::DefenseState.bits(),
    DojangBerserk(()) => CharSecondaryStatFlags::DojangBerserk.bits(),
    DojangInvincible(()) => CharSecondaryStatFlags::DojangInvincible.bits(),
    WindWalk(()) => CharSecondaryStatFlags::WindWalk.bits(),
    // r
    RepeatEffect(u32) => CharSecondaryStatFlags::RepeatEffect.bits(),
    // r
    StopPortion(u32) => CharSecondaryStatFlags::StopPortion.bits(),
    // r
    StopMotion(u32) => CharSecondaryStatFlags::StopMotion.bits(),
    // r
    Fear(u32) => CharSecondaryStatFlags::Fear.bits(),
    // r
    MagicShield(u32) => CharSecondaryStatFlags::MagicShield.bits(),
    Flying(()) => CharSecondaryStatFlags::Flying.bits(),
    // r
    Frozen(u32) => CharSecondaryStatFlags::Frozen.bits(),
    // r
    SuddenDeath(u32) => CharSecondaryStatFlags::SuddenDeath.bits(),
    // r
    FinalCut(u32) => CharSecondaryStatFlags::FinalCut.bits(),
    // n
    Cyclone(u8) => CharSecondaryStatFlags::Cyclone.bits(),
    Sneak(()) => CharSecondaryStatFlags::Sneak.bits(),
    MorewildDamageUp(()) => CharSecondaryStatFlags::MorewildDamageUp.bits(),
    // r
    Mechanic(u32) => CharSecondaryStatFlags::Mechanic.bits(),
    // r
    DarkAura(u32) => CharSecondaryStatFlags::DarkAura.bits(),
    // r
    BlueAura(u32) => CharSecondaryStatFlags::BlueAura.bits(),
    // r
    YellowAura(u32) => CharSecondaryStatFlags::YellowAura.bits(),
    BlessingArmor(()) => CharSecondaryStatFlags::BlessingArmor.bits(),
);

pub struct RemoteCharSecondaryStatExtra {
    pub defense_att: u8, //n
    pub defense_state: u8,
    /*
       TempStatBases, see SecondaryStat
    */
}

#[derive(ShroomPacket, Debug, Default, Clone)]
pub struct ExpireTerm {
    pub term: u16,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct TwoStateBase {
    pub value: u32,
    pub reason: u32,
    pub last_updated: ClientTimeOffset,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct TwoStateBase2 {
    pub value: u32,
    pub reason: u32,
    pub negative: bool,
    pub time: u32,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct TwoStateNoExpire {
    pub base: TwoStateBase,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct TwoStateExpireLastUpdated {
    pub base: TwoStateBase,
    pub expire_term: ExpireTerm,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct TwoStateExpireCurrentTime {
    pub base: TwoStateBase,
    pub current_time: ClientTimeOffset,
    pub expire_term: ExpireTerm,
}

#[derive(ShroomPacket, Debug, Clone)]
pub struct TwoStateGuidedBullet {
    pub base: TwoStateBase,
    pub mob_id: MobId,
}

partial_data!(
    CharSecondaryStat,
    CharSecondaryStatFlags,
    u128,
    derive(Debug, Clone),
    Pad(TempStatValue) =>  1 << 0,
    Pdd(TempStatValue) =>  1 << 1,
    Mad(TempStatValue) =>  1 << 2,
    Mdd(TempStatValue) =>  1 << 3,
    Acc(TempStatValue) =>  1 << 4,
    Evasion(TempStatValue) =>  1 << 5,
    CriticalRate(TempStatValue) =>  1 << 6,
    Speed(TempStatValue) =>  1 << 7,
    Jump(TempStatValue) =>  1 << 8,
    ExtraMaxHp(TempStatValue) =>  1 << 0x5D,
    ExtraMaxMp(TempStatValue) =>  1 << 0x5E,
    ExtraPad(TempStatValue) =>  1 << 0x5F,
    ExtraPdd(TempStatValue) =>  1 << 0x60,
    ExtraMdd(TempStatValue) =>  1 << 0x61,
    MagicGuard(TempStatValue) =>  1 << 9,
    DarkSight(TempStatValue) =>  1 << 0xa,
    Booster(TempStatValue) =>  1 << 0xb,
    PowerGuard(TempStatValue) =>  1 << 0xc,
    Guard(TempStatValue) =>  1 << 0x62,
    SafetyDamage(TempStatValue) =>  1 << 0x63,
    SafetyAbsorb(TempStatValue) =>  1 << 0x64,
    MaxHp(TempStatValue) =>  1 << 0xd,
    MaxMp(TempStatValue) =>  1 << 0xe,
    Invincible(TempStatValue) =>  1 << 0xf,
    SoulArrow(TempStatValue) =>  1 << 0x10,
    Stun(TempStatValue) =>  1 << 0x11,
    Poison(TempStatValue) =>  1 << 0x12,
    Seal(TempStatValue) =>  1 << 0x13,
    Darkness(TempStatValue) =>  1 << 0x14,
    ComboCounter(TempStatValue) =>  1 << 0x15,
    WeaponCharge(TempStatValue) =>  1 << 0x16,
    DragonBlood(TempStatValue) =>  1 << 0x17,
    HolySymbol(TempStatValue) =>  1 << 0x18,
    MesoUp(TempStatValue) =>  1 << 0x19,
    ShadowPartner(TempStatValue) =>  1 << 0x1A,
    PickPocket(TempStatValue) =>  1 << 0x1B,
    MesoGuard(TempStatValue) =>  1 << 0x1C,
    Thaw(TempStatValue) =>  1 << 0x1D,
    Weakness(TempStatValue) =>  1 << 0x1E,
    Curse(TempStatValue) =>  1 << 0x1F,
    Slow(TempStatValue) =>  1 << 0x20, // Done
    Morph(TempStatValue) =>  1 << 0x21,
    Ghost(TempStatValue) =>  1 << 0x31, // ghost morph
    Regen(TempStatValue) =>  1 << 0x22, // recovery
    BasicStatUp(TempStatValue) =>  1 << 0x23, // shroom warrior
    Stance(TempStatValue) =>  1 << 0x24, // Done
    SharpEyes(TempStatValue) =>  1 << 0x25, // Done
    ManaReflection(TempStatValue) =>  1 << 0x26, // Done
    Attract(TempStatValue) =>  1 << 0x27,  // seduce
    SpiritJavelin(TempStatValue) =>  1 << 0x28, // shadow claw
    Infinity(TempStatValue) =>  1 << 0x29, // Done
    Holyshield(TempStatValue) =>  1 << 0x2A, // Done
    HamString(TempStatValue) =>  1 << 0x2B, // Done
    Blind(TempStatValue) =>  1 << 0x2C, // Done
    Concentration(TempStatValue) =>  1 << 0x2D, // Done
    BanMap(TempStatValue) =>  1 << 0x2E,
    MaxLevelBuff(TempStatValue) =>  1 << 0x2F, // echo of hero
    Barrier(TempStatValue) =>  1 << 0x32,
    DojangShield(TempStatValue) =>  1 << 0x3E,
    ReverseInput(TempStatValue) =>  1 << 0x33, // confuse
    MesoUpByItem(TempStatValue) =>  1 << 0x30, // Done
    ItemUpByItem(TempStatValue) =>  1 << 0x34, // Done
    RespectPImmune(TempStatValue) =>  1 << 0x35,
    RespectMImmune(TempStatValue) =>  1 << 0x36,
    DefenseAtt(TempStatValue) =>  1 << 0x37,
    DefenseState(TempStatValue) =>  1 << 0x38,
    DojangBerserk(TempStatValue) =>  1 << 0x3B, // berserk fury
    DojangInvincible(TempStatValue) =>  1 << 0x3C, // divine body
    Spark(TempStatValue) =>  1 << 0x3D, // Done
    SoulMasterFinal(TempStatValue) =>  1 << 0x3F, // Done ?
    WindBreakerFinal(TempStatValue) =>  1 << 0x40, // Done ?
    ElementalReset(TempStatValue) =>  1 << 0x41, // Done
    WindWalk(TempStatValue) =>  1 << 0x42, // Done
    EventRate(TempStatValue) =>  1 << 0x43,
    ComboAbilityBuff(TempStatValue) =>  1 << 0x44, // aran combo
    ComboDrain(TempStatValue) =>  1 << 0x45, // Done
    ComboBarrier(TempStatValue) =>  1 << 0x46, // Done
    BodyPressure(TempStatValue) =>  1 << 0x47, // Done
    SmartKnockback(TempStatValue) =>  1 << 0x48, // Done
    RepeatEffect(TempStatValue) =>  1 << 0x49,
    ExpBuffRate(TempStatValue) =>  1 << 0x4A, // Done
    IncEffectHPPotion(TempStatValue) =>  1 << 0x39,
    IncEffectMPPotion(TempStatValue) =>  1 << 0x3A,
    StopPortion(TempStatValue) =>  1 << 0x4B,
    StopMotion(TempStatValue) =>  1 << 0x4C,
    Fear(TempStatValue) =>  1 << 0x4D, // debuff done
    EvanSlow(TempStatValue) =>  1 << 0x4E, // Done
    MagicShield(TempStatValue) =>  1 << 0x4F, // Done
    MagicResistance(TempStatValue) =>  1 << 0x50, // Done
    SoulStone(TempStatValue) =>  1 << 0x51,
    Flying(TempStatValue) =>  1 << 0x52,
    Frozen(TempStatValue) =>  1 << 0x53,
    AssistCharge(TempStatValue) =>  1 << 0x54,
    Enrage(TempStatValue) =>  1 << 0x55, //mirror imaging
    SuddenDeath(TempStatValue) =>  1 << 0x56,
    NotDamaged(TempStatValue) =>  1 << 0x57,
    FinalCut(TempStatValue) =>  1 << 0x58,
    ThornsEffect(TempStatValue) =>  1 << 0x59,
    SwallowAttackDamage(TempStatValue) =>  1 << 0x5A,
    MorewildDamageUp(TempStatValue) =>  1 << 0x5B,
    Mine(TempStatValue) =>  1 << 0x5C,
    Cyclone(TempStatValue) =>  1 << 0x65,
    SwallowCritical(TempStatValue) =>  1 << 0x66,
    SwallowMaxMP(TempStatValue) =>  1 << 0x67,
    SwallowDefence(TempStatValue) =>  1 << 0x68,
    SwallowEvasion(TempStatValue) =>  1 << 0x69,
    Conversion(TempStatValue) =>  1 << 0x6A,
    Revive(TempStatValue) =>  1 << 0x6B, // summon reaper buff
    Sneak(TempStatValue) =>  1 << 0x6C,
    Mechanic(TempStatValue) =>  1 << 0x6D,
    Aura(TempStatValue) =>  1 << 0x6E,
    DarkAura(TempStatValue) =>  1 << 0x6F,
    BlueAura(TempStatValue) =>  1 << 0x70,
    YellowAura(TempStatValue) =>  1 << 0x71,
    SuperBody(TempStatValue) =>  1 << 0x72, // body boost
    MorewildMaxHP(TempStatValue) =>  1 << 0x73,
    Dice(TempStatValue) =>  1 << 0x74,
    BlessingArmor(TempStatValue) =>  1 << 0x75, // Paladin Divine Shield
    DamR(TempStatValue) =>  1 << 0x76,
    TeleportMasteryOn(TempStatValue) =>  1 << 0x77,
    CombatOrders(TempStatValue) =>  1 << 0x78,
    Beholder(TempStatValue) =>  1 << 0x79,

    // Temp two state
    EnergyCharged(()) => 1 << 0x7A, // TODO What does >= 10_000 mean
    DashSpeed(()) => 1 << 0x7B,
    DashJump(()) => 1 << 0x7C,
    RideVehicle(()) => 1 << 0x7D,
    PartyBooster(()) => 1 << 0x7E,
    GuidedBullet(()) => 1 << 0x7F,

    /*
        TODO
        Cause Shroom Coders are trolls they also added those:
        (1 << 128 overflows a 128 bit int)

        Undead(TwoStateExpireLastUpdated) => 1 << 0x80,
        SummonBomb(TempStatValue) => 1 << 0x81,
    */
);

partial_data!(
    CharSecondaryTwoStates,
    CharSecondaryTwoStatesFlags,
    u128,
    derive(Debug, Clone),
    EnergyCharged(TwoStateNoExpire) => 1 << 0x7A, // TODO What does >= 10_000 mean
    DashSpeed(TwoStateExpireLastUpdated) => 1 << 0x7B,
    DashJump(TwoStateExpireLastUpdated) => 1 << 0x7C,
    RideVehicle(TwoStateBase) => 1 << 0x7D,
    PartyBooster(TwoStateExpireCurrentTime) => 1 << 0x7E,
    GuidedBullet(TwoStateGuidedBullet) => 1 << 0x7F,
);

pub type DiceInfo = [u32; 0x16];

#[derive(Debug)]
pub struct LocalSecondaryStatSetResp {
    pub stats: PartialFlag<(), CharSecondaryStatPartial>,
    pub defense_atk: u8,
    pub defense_state: u8,
    pub swallow_buff_time: CondOption<u8>,
    pub dice_info: CondOption<DiceInfo>,
    pub blessing_armor_inc_pad: CondOption<u32>,
    pub two_states: CharSecondaryTwoStatesPartial,
    pub delay: ShroomDurationMs16,
    pub movement_affecting: CondOption<bool>,
}

impl CharSecondaryStatFlags {
    fn is_movement_affecting(&self) -> bool {
        self.contains(
            Self::Speed
                | Self::Jump
                | Self::Stun
                | Self::Weakness
                | Self::Slow
                | Self::Morph
                | Self::Ghost
                | Self::BasicStatUp
                | Self::Attract
                | Self::RideVehicle
                | Self::Flying
                | Self::Frozen
                | Self::DashSpeed
                | Self::DashJump
                | Self::PartyBooster
                | Self::YellowAura,
        )
    }

    fn has_swallow_stats(&self) -> bool {
        self.contains(
            Self::SwallowAttackDamage
                | Self::SwallowCritical
                | Self::SwallowMaxMP
                | Self::SwallowDefence
                | Self::SwallowEvasion,
        )
    }

    pub fn to_two_state_flags(&self) -> CharSecondaryTwoStatesFlags {
        CharSecondaryTwoStatesFlags::from_bits_truncate(self.bits())
    }
}

impl<'de> shroom_pkt::DecodePacket<'de> for LocalSecondaryStatSetResp {
    fn decode(pr: &mut shroom_pkt::PacketReader<'de>) -> shroom_pkt::PacketResult<Self> {
        let stats = PartialFlag::<(), CharSecondaryStatPartial>::decode(pr)?;
        let flags = stats.data.get_flags();

        Ok(Self {
            stats,
            defense_atk: pr.read_u8()?,
            defense_state: pr.read_u8()?,
            swallow_buff_time: CondOption::decode_cond(flags.has_swallow_stats(), pr)?,
            dice_info: CondOption::decode_cond(flags.has_dice(), pr)?,
            blessing_armor_inc_pad: CondOption::decode_cond(flags.has_blessingarmor(), pr)?,
            two_states: CharSecondaryTwoStatesPartial::partial_decode(
                CharSecondaryTwoStatesFlags::from_bits_truncate(flags.bits()),
                pr,
            )?,
            delay: ShroomDurationMs16::decode(pr)?,
            movement_affecting: CondOption::decode_cond(flags.is_movement_affecting(), pr)?,
        })
    }
}

impl shroom_pkt::EncodePacket for LocalSecondaryStatSetResp {
    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::NONE;

    fn encode_len(&self) -> usize {
        self.stats.encode_len()
            + self.defense_atk.encode_len()
            + self.defense_state.encode_len()
            + self.swallow_buff_time.encode_len()
            + self.dice_info.encode_len()
            + self.blessing_armor_inc_pad.encode_len()
            + self
                .two_states
                .partial_encode_len(self.stats.data.get_flags().to_two_state_flags())
            + self.delay.encode_len()
            + self.movement_affecting.encode_len()
    }

    fn encode<T: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<T>,
    ) -> shroom_pkt::PacketResult<()> {
        self.stats.encode(pw)?;
        self.defense_atk.encode(pw)?;
        self.defense_state.encode(pw)?;

        // Check for swallow
        self.swallow_buff_time.encode(pw)?;
        self.dice_info.encode(pw)?;
        self.blessing_armor_inc_pad.encode(pw)?;
        self.two_states
            .partial_encode(self.stats.data.get_flags().to_two_state_flags(), pw)?;
        self.delay.encode(pw)?;
        self.movement_affecting.encode(pw)?;

        Ok(())
    }
}

with_opcode!(LocalSecondaryStatSetResp, SendOpcodes::TemporaryStatSet);

#[derive(Debug, ShroomPacket)]
pub struct LocalSecondaryStatSetResp2<T> {
    pub stats: T,
    pub delay: ShroomDurationMs16,
    pub movement_affecting: OptionTail<bool>,
}

impl<T> HasOpCode for LocalSecondaryStatSetResp2<T> {
    type OpCode = SendOpcodes;
    const OPCODE: SendOpcodes = SendOpcodes::TemporaryStatSet;
}

#[derive(ShroomPacket)]
pub struct LocalSecondaryStatResetResp {
    pub flags: CharSecondaryStatFlags,
    pub movement_affecting: bool, // Should be an option
}
with_opcode!(LocalSecondaryStatResetResp, SendOpcodes::TemporaryStatReset);

#[cfg(test)]
mod tests {
    use shroom_pkt::{DecodePacket, PacketReader};

    use super::*;

    #[test]
    fn partial_stat_set() {
        let data = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x0C, 0x00, 0xE9, 0x03, 0x00, 0x00, 0x2F, 0x75, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00,
        ];
        let partial_data =
            LocalSecondaryStatSetResp::decode_complete(&mut PacketReader::new(&data)).unwrap();
        dbg!(&partial_data);
    }
}
