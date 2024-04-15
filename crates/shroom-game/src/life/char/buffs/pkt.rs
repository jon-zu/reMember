use bytes::BufMut;
use shroom_meta::buffs::{
    char::{CharBuff, CharBuffStat, CharBuffValue, DashSpeed, EnergyCharged}, keys::CharBuffKey, BuffKey
};
use shroom_pkt::{
    time::{ClientTimeOffset, DurationMs},
    EncodePacket, PacketResult, PacketWriter, SizeHint,
};
use shroom_proto95::game::user::secondary_stats::{
    CharSecondaryStatFlags, ExpireTerm, TempStatValue, TwoStateBase, TwoStateExpireCurrentTime,
    TwoStateExpireLastUpdated, TwoStateGuidedBullet,
};
use shroom_srv::GameTime;

use super::CharBuffs;

pub struct CharBuffPacket<'a> {
    pub buffs: &'a CharBuffs,
    pub flags: CharSecondaryStatFlags,
    pub t: GameTime,
}

impl<'a> CharBuffPacket<'a> {
    #[inline]
    fn encode_if_flag<T: CharBuffStat + CharBuffValue, B: BufMut>(
        &self,
        pw: &mut PacketWriter<B>,
        t: GameTime,
    ) -> PacketResult<()> {

        let f = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());
        if !self.flags.contains(f) {
            return Ok(());
        }

        let b = T::get(&self.buffs.storage);
        TempStatValue {
            value: b.data.to_buff_value(),
            duration: self.buffs.exp.expiration_dur(T::KEY, t).into(),
            reason: b.id.to_src32(),
        }
        .encode(pw)?;
        Ok(())
    }

    fn get_last_updated<T: CharBuffStat>(
        &self,
        f: impl FnOnce(&T) -> u32,
    ) -> TwoStateExpireLastUpdated {
        let b = &T::get(&self.buffs.storage);
        let dur = self.buffs.exp.expiration_dur(T::KEY, self.t);
        TwoStateExpireLastUpdated {
            base: TwoStateBase {
                value: f(&b.data),
                reason: b.id.to_src32(),
                last_updated: ClientTimeOffset(DurationMs(dur.as_millis() as i32)),
            },
            expire_term: ExpireTerm {
                term: dur.as_secs() as u16,
            },
        }
    }

    fn encode_two_states<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        let f = &self.flags;
        let st = &self.buffs.storage;
        if f.contains(CharSecondaryStatFlags::EnergyCharged) {
            self.get_last_updated::<EnergyCharged>(|e| e.energy)
                .encode(pw)?;
        }

        if f.contains(CharSecondaryStatFlags::DashSpeed) {
            self.get_last_updated::<DashSpeed>(|b| b.0 as u32)
                .encode(pw)?;
        }

        if f.contains(CharSecondaryStatFlags::DashJump) {
            self.get_last_updated::<DashSpeed>(|b| b.0 as u32)
                .encode(pw)?;
        }

        if f.contains(CharSecondaryStatFlags::RideVehicle) {
            TwoStateBase {
                value: st.RideVehicle.data.0,
                reason: st.RideVehicle.id.to_src32(),
                last_updated: ClientTimeOffset(DurationMs(-10000)),
            }
            .encode(pw)?;
        }

        if f.contains(CharSecondaryStatFlags::PartyBooster) {
            let b = &st.PartyBooster;
            let exp = self
                .buffs
                .exp
                .expiration_dur(CharBuffKey::PartyBooster, self.t);

            TwoStateExpireCurrentTime {
                base: TwoStateBase {
                    value: b.data.0 as u32,
                    reason: b.id.to_src32(),
                    last_updated: ClientTimeOffset(DurationMs(exp.as_millis() as i32)),
                },
                current_time: ClientTimeOffset(DurationMs(0)),
                expire_term: ExpireTerm {
                    term: exp.as_secs() as u16,
                },
            }
            .encode(pw)?;
        }

        if f.contains(CharSecondaryStatFlags::GuidedBullet) {
            let b = &st.GuidedBullet;
            TwoStateGuidedBullet {
                base: TwoStateBase {
                    value: b.data.value,
                    reason: b.id.to_src32(),
                    last_updated: ClientTimeOffset(DurationMs(-1000)),
                },
                mob_id: b.data.mob_id.0.into(),
            }
            .encode(pw)?;
        }

        Ok(())
    }

    fn encode_def_atk_state_attr<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        if self.flags.contains(CharSecondaryStatFlags::DefenseAtt) {
            (self.buffs.storage.DefenseAtt.data.attr).encode(pw)?;
        } else {
            (0u8).encode(pw)?;
        }

        if self.flags.contains(CharSecondaryStatFlags::DefenseState) {
            (self.buffs.storage.DefenseState.data.attr).encode(pw)?;
        } else {
            (0u8).encode(pw)?;
        }
        Ok(())
    }
}

impl<'a> EncodePacket for CharBuffPacket<'a> {
    const SIZE_HINT: shroom_pkt::SizeHint = shroom_pkt::SizeHint::NONE;

    fn encode<B: bytes::BufMut>(
        &self,
        pw: &mut shroom_pkt::PacketWriter<B>,
    ) -> shroom_pkt::PacketResult<()> {
        use shroom_meta::buffs::char::*;
        let st = &self.buffs.storage;
        let f = self.flags.clone();
        f.encode(pw)?;

        self.encode_if_flag::<Pad, _>(pw, self.t)?;
        self.encode_if_flag::<Pdd, _>(pw, self.t)?;
        self.encode_if_flag::<Mad, _>(pw, self.t)?;
        self.encode_if_flag::<Mdd, _>(pw, self.t)?;
        self.encode_if_flag::<Acc, _>(pw, self.t)?;
        self.encode_if_flag::<Evasion, _>(pw, self.t)?;
        self.encode_if_flag::<CriticalRate, _>(pw, self.t)?;
        self.encode_if_flag::<Speed, _>(pw, self.t)?;
        self.encode_if_flag::<Jump, _>(pw, self.t)?;
        self.encode_if_flag::<ExtraMaxHp, _>(pw, self.t)?;
        self.encode_if_flag::<ExtraMaxMp, _>(pw, self.t)?;
        self.encode_if_flag::<ExtraPad, _>(pw, self.t)?;
        self.encode_if_flag::<ExtraPdd, _>(pw, self.t)?;
        self.encode_if_flag::<ExtraMdd, _>(pw, self.t)?;
        self.encode_if_flag::<MagicGuard, _>(pw, self.t)?;
        self.encode_if_flag::<DarkSight, _>(pw, self.t)?;
        self.encode_if_flag::<Booster, _>(pw, self.t)?;
        self.encode_if_flag::<PowerGuard, _>(pw, self.t)?;
        self.encode_if_flag::<Guard, _>(pw, self.t)?;
        self.encode_if_flag::<SafetyDamage, _>(pw, self.t)?;
        self.encode_if_flag::<SafetyAbsorb, _>(pw, self.t)?;
        self.encode_if_flag::<MaxHp, _>(pw, self.t)?;
        self.encode_if_flag::<MaxMp, _>(pw, self.t)?;
        self.encode_if_flag::<Invincible, _>(pw, self.t)?;
        self.encode_if_flag::<SoulArrow, _>(pw, self.t)?;
        self.encode_if_flag::<Stun, _>(pw, self.t)?;
        self.encode_if_flag::<Poison, _>(pw, self.t)?;
        self.encode_if_flag::<Seal, _>(pw, self.t)?;
        self.encode_if_flag::<Darkness, _>(pw, self.t)?;
        self.encode_if_flag::<ComboCounter, _>(pw, self.t)?;
        self.encode_if_flag::<WeaponCharge, _>(pw, self.t)?;
        self.encode_if_flag::<DragonBlood, _>(pw, self.t)?;
        self.encode_if_flag::<HolySymbol, _>(pw, self.t)?;
        self.encode_if_flag::<MesoUp, _>(pw, self.t)?;
        self.encode_if_flag::<ShadowPartner, _>(pw, self.t)?;
        self.encode_if_flag::<PickPocket, _>(pw, self.t)?;
        self.encode_if_flag::<MesoGuard, _>(pw, self.t)?;
        self.encode_if_flag::<Thaw, _>(pw, self.t)?;
        self.encode_if_flag::<Weakness, _>(pw, self.t)?;
        self.encode_if_flag::<Curse, _>(pw, self.t)?;
        self.encode_if_flag::<Slow, _>(pw, self.t)?;
        self.encode_if_flag::<Morph, _>(pw, self.t)?;
        self.encode_if_flag::<Ghost, _>(pw, self.t)?;
        self.encode_if_flag::<Regen, _>(pw, self.t)?;
        self.encode_if_flag::<BasicStatUp, _>(pw, self.t)?;
        self.encode_if_flag::<Stance, _>(pw, self.t)?;
        self.encode_if_flag::<SharpEyes, _>(pw, self.t)?;
        self.encode_if_flag::<ManaReflection, _>(pw, self.t)?;
        self.encode_if_flag::<Attract, _>(pw, self.t)?;
        self.encode_if_flag::<SpiritJavelin, _>(pw, self.t)?;
        self.encode_if_flag::<Infinity, _>(pw, self.t)?;
        self.encode_if_flag::<Holyshield, _>(pw, self.t)?;
        self.encode_if_flag::<HamString, _>(pw, self.t)?;
        self.encode_if_flag::<Blind, _>(pw, self.t)?;
        self.encode_if_flag::<Concentration, _>(pw, self.t)?;
        self.encode_if_flag::<BanMap, _>(pw, self.t)?;
        self.encode_if_flag::<MaxLevelBuff, _>(pw, self.t)?;
        self.encode_if_flag::<Barrier, _>(pw, self.t)?;
        self.encode_if_flag::<DojangShield, _>(pw, self.t)?;
        self.encode_if_flag::<ReverseInput, _>(pw, self.t)?;
        self.encode_if_flag::<MesoUpByItem, _>(pw, self.t)?;
        self.encode_if_flag::<ItemUpByItem, _>(pw, self.t)?;
        self.encode_if_flag::<RespectPImmune, _>(pw, self.t)?;
        self.encode_if_flag::<RespectMImmune, _>(pw, self.t)?;
        self.encode_if_flag::<DefenseAtt, _>(pw, self.t)?;
        self.encode_if_flag::<DefenseState, _>(pw, self.t)?;
        self.encode_if_flag::<DojangBerserk, _>(pw, self.t)?;
        self.encode_if_flag::<DojangInvincible, _>(pw, self.t)?;
        self.encode_if_flag::<Spark, _>(pw, self.t)?;
        self.encode_if_flag::<SoulMasterFinal, _>(pw, self.t)?;
        self.encode_if_flag::<WindBreakerFinal, _>(pw, self.t)?;
        self.encode_if_flag::<ElementalReset, _>(pw, self.t)?;
        self.encode_if_flag::<WindWalk, _>(pw, self.t)?;
        self.encode_if_flag::<EventRate, _>(pw, self.t)?;
        self.encode_if_flag::<ComboAbilityBuff, _>(pw, self.t)?;
        self.encode_if_flag::<ComboDrain, _>(pw, self.t)?;
        self.encode_if_flag::<ComboBarrier, _>(pw, self.t)?;
        self.encode_if_flag::<BodyPressure, _>(pw, self.t)?;
        self.encode_if_flag::<SmartKnockback, _>(pw, self.t)?;
        self.encode_if_flag::<RepeatEffect, _>(pw, self.t)?;
        self.encode_if_flag::<ExpBuffRate, _>(pw, self.t)?;
        self.encode_if_flag::<IncEffectHPPotion, _>(pw, self.t)?;
        self.encode_if_flag::<IncEffectMPPotion, _>(pw, self.t)?;
        self.encode_if_flag::<StopPortion, _>(pw, self.t)?;
        self.encode_if_flag::<StopMotion, _>(pw, self.t)?;
        self.encode_if_flag::<Fear, _>(pw, self.t)?;
        self.encode_if_flag::<EvanSlow, _>(pw, self.t)?;
        self.encode_if_flag::<MagicShield, _>(pw, self.t)?;
        self.encode_if_flag::<MagicResistance, _>(pw, self.t)?;
        self.encode_if_flag::<SoulStone, _>(pw, self.t)?;
        self.encode_if_flag::<Flying, _>(pw, self.t)?;
        self.encode_if_flag::<Frozen, _>(pw, self.t)?;
        self.encode_if_flag::<AssistCharge, _>(pw, self.t)?;
        self.encode_if_flag::<Enrage, _>(pw, self.t)?;
        self.encode_if_flag::<SuddenDeath, _>(pw, self.t)?;
        self.encode_if_flag::<NotDamaged, _>(pw, self.t)?;
        self.encode_if_flag::<FinalCut, _>(pw, self.t)?;
        self.encode_if_flag::<ThornsEffect, _>(pw, self.t)?;
        self.encode_if_flag::<SwallowAttackDamage, _>(pw, self.t)?;
        self.encode_if_flag::<MorewildDamageUp, _>(pw, self.t)?;
        self.encode_if_flag::<Mine, _>(pw, self.t)?;
        self.encode_if_flag::<Cyclone, _>(pw, self.t)?;
        self.encode_if_flag::<SwallowCritical, _>(pw, self.t)?;
        self.encode_if_flag::<SwallowMaxMP, _>(pw, self.t)?;
        self.encode_if_flag::<SwallowDefence, _>(pw, self.t)?;
        self.encode_if_flag::<SwallowEvasion, _>(pw, self.t)?;
        self.encode_if_flag::<Conversion, _>(pw, self.t)?;
        self.encode_if_flag::<Revive, _>(pw, self.t)?;
        self.encode_if_flag::<Sneak, _>(pw, self.t)?;
        self.encode_if_flag::<Mechanic, _>(pw, self.t)?;
        self.encode_if_flag::<Aura, _>(pw, self.t)?;
        self.encode_if_flag::<DarkAura, _>(pw, self.t)?;
        self.encode_if_flag::<BlueAura, _>(pw, self.t)?;
        self.encode_if_flag::<YellowAura, _>(pw, self.t)?;
        self.encode_if_flag::<SuperBody, _>(pw, self.t)?;
        self.encode_if_flag::<MorewildMaxHP, _>(pw, self.t)?;
        let dice = f.contains(CharSecondaryStatFlags::Dice);
        self.encode_if_flag::<Dice, _>(pw, self.t)?;
        let blessing_armor = f.contains(CharSecondaryStatFlags::BlessingArmor);
        self.encode_if_flag::<BlessingArmor, _>(pw, self.t)?;
        self.encode_if_flag::<DamR, _>(pw, self.t)?;
        self.encode_if_flag::<TeleportMasteryOn, _>(pw, self.t)?;
        self.encode_if_flag::<CombatOrders, _>(pw, self.t)?;
        self.encode_if_flag::<Beholder, _>(pw, self.t)?;

        // Def Atk
        self.encode_def_atk_state_attr(pw)?;

        // TODO can depend on any swallow flags being set
        // TODO swallow should be grouped
        if f.contains(CharSecondaryStatFlags::SwallowAttackDamage) {
            //TODO swallow time
            (st.SwallowAttackDamage.data.time as u32).encode(pw)?;
        }

        if dice {
            st.Dice.data.stats.encode(pw)?;
        }

        if blessing_armor {
            (st.BlessingArmor.data.pad).encode(pw)?;
        }

        self.encode_two_states(pw)?;

        // TODO: delay u16 ms not really a part
        // TODO movement affecting bool if flags are affecting the movement also not really a part
        Ok(())
    }
}

pub struct CharBuffRemotePacket<'a>(CharBuffPacket<'a>);

impl<'a> CharBuffRemotePacket<'a> {
    pub fn get<T: CharBuffStat>(&self) -> Option<&CharBuff<T>> {
        let f = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());
        if self.0.flags.contains(f) {
            Some(T::get(&self.0.buffs.storage))
        } else {
            None
        }
    }
}

impl<'a> EncodePacket for CharBuffRemotePacket<'a> {
    const SIZE_HINT: shroom_pkt::SizeHint = SizeHint::NONE;

    fn encode<B: BufMut>(&self, pw: &mut PacketWriter<B>) -> PacketResult<()> {
        use shroom_meta::buffs::char::*;
        let f = self.0.flags.clone();
        f.encode(pw)?;

        if let Some(b) = self.get::<Speed>() {
            (b.data.0 as u8).encode(pw)?;
        }

        if let Some(b) = self.get::<ComboCounter>() {
            (b.data.orbs as u8).encode(pw)?;
        }

        if let Some(b) = self.get::<WeaponCharge>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Stun>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Darkness>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Seal>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Weakness>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Curse>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Poison>() {
            (b.data.0).encode(pw)?;
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<ShadowPartner>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Morph>() {
            (b.data.0).encode(pw)?;
        }

        if let Some(b) = self.get::<Ghost>() {
            (b.data.0).encode(pw)?;
        }

        if let Some(b) = self.get::<Attract>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<SpiritJavelin>() {
            (b.data.to_buff_value()).encode(pw)?;
        }

        if let Some(b) = self.get::<BanMap>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Barrier>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<DojangShield>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<ReverseInput>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<RespectPImmune>() {
            (b.data.0).encode(pw)?;
        }

        if let Some(b) = self.get::<RespectMImmune>() {
            (b.data.0).encode(pw)?;
        }

        if let Some(b) = self.get::<DefenseAtt>() {
            (b.data.value).encode(pw)?;
        }

        if let Some(b) = self.get::<DefenseState>() {
            (b.data.value).encode(pw)?;
        }

        if let Some(b) = self.get::<RepeatEffect>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<StopPortion>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<StopMotion>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Fear>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<MagicShield>() {
            (b.data.0).encode(pw)?;
        }

        if let Some(b) = self.get::<Frozen>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<SuddenDeath>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<FinalCut>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<Cyclone>() {
            (b.data.0 as u8).encode(pw)?;
        }

        if let Some(b) = self.get::<Mechanic>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<DarkAura>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<BlueAura>() {
            (b.id).encode(pw)?;
        }

        if let Some(b) = self.get::<YellowAura>() {
            (b.id).encode(pw)?;
        }

        self.0.encode_def_atk_state_attr(pw)?;

        self.0.encode_two_states(pw)?;
        Ok(())
    }
}
