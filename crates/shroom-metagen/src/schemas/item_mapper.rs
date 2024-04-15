use super::shroom_schemas::{self as sch, CharItemInfoAddition, ItemValue};
use super::{IntoBool, IntoNum};
use crate::schemas::quest_mapper::map_to_list;
use crate::schemas::shroom_schemas::{ItemValueInfo, StrOrInt};
use shroom_meta::id::item_id::ItemType;
use shroom_meta::id::job_id::JobId;
use shroom_meta::id::{FieldId, ItemId, ItemOptionId, MobId, Money, QuestId, SkillId};
use shroom_meta::item::{
    EquipBaseStats, EquipStat, EquipStats, ItemStat, ItemStatRatio, JobFlag, ScrollChance,
};
use shroom_meta::skill::SkillLevel;
use shroom_meta::tmpl::equip::*;
use shroom_meta::tmpl::item::*;
use shroom_meta::{CharLevel, Pop, ProcChance};
use std::time::Duration;

fn item_stat(v: &impl IntoNum) -> ItemStat {
    ItemStat(v.into_num() as u16)
}

fn item_stat_r(v: &impl IntoNum) -> ItemStatRatio {
    ItemStatRatio(v.into_num() as u16)
}

macro_rules! map_base_ratio_stats {
    ($v:ident) => {
        enum_map::EnumMap::from_fn(|stat| match stat {
            EquipStat::Str => item_stat_r(&$v.inc_st_rr),
            EquipStat::Dex => item_stat_r(&$v.inc_de_xr),
            EquipStat::Int => item_stat_r(&$v.inc_in_tr),
            EquipStat::Luk => item_stat_r(&$v.inc_lu_kr),
            EquipStat::Hp => item_stat_r(&$v.inc_mh_pr),
            EquipStat::Mp => item_stat_r(&$v.inc_mm_pr),
            EquipStat::Pad => item_stat_r(&$v.inc_pa_dr),
            EquipStat::Mad => item_stat_r(&$v.inc_ma_dr),
            EquipStat::Pdd => item_stat_r(&$v.inc_pd_dr),
            EquipStat::Mdd => item_stat_r(&$v.inc_md_dr),
            EquipStat::Acc => item_stat_r(&$v.inc_ac_cr),
            EquipStat::Eva => item_stat_r(&$v.inc_ev_ar),
            _ => item_stat_r(&0),
        })
    };
}

macro_rules! map_base_stats {
    ($v:ident, $craft:ident) => {
        EquipBaseStats(enum_map::EnumMap::from_fn(|stat| match stat {
            EquipStat::Str => item_stat(&$v.inc_str),
            EquipStat::Dex => item_stat(&$v.inc_dex),
            EquipStat::Int => item_stat(&$v.inc_int),
            EquipStat::Luk => item_stat(&$v.inc_luk),
            EquipStat::Hp => item_stat(&$v.inc_mhp),
            EquipStat::Mp => item_stat(&$v.inc_mmp),
            EquipStat::Pad => item_stat(&$v.inc_pad),
            EquipStat::Mad => item_stat(&$v.inc_mad),
            EquipStat::Pdd => item_stat(&$v.inc_pdd),
            EquipStat::Mdd => item_stat(&$v.inc_mdd),
            EquipStat::Acc => item_stat(&$v.inc_acc),
            EquipStat::Eva => item_stat(&$v.inc_eva),
            EquipStat::Craft => item_stat(&$v.$craft),
            EquipStat::Speed => item_stat(&$v.inc_speed),
            EquipStat::Jump => item_stat(&$v.inc_jump),
        }))
    };
}

impl TryFrom<&sch::CharItemInfo> for EquipStats {
    type Error = anyhow::Error;

    fn try_from(value: &sch::CharItemInfo) -> Result<Self, Self::Error> {
        let base = map_base_stats!(value, inc_craft);

        Ok(Self {
            base,
            inc_max_hp_r: ItemStatRatio(value.inc_mh_pr.into_num() as u16),
            inc_max_mp_r: ItemStatRatio(value.inc_mm_pr.into_num() as u16),
        })
    }
}

impl TryFrom<&sch::CharItemInfo> for EquipItemFlags {
    type Error = anyhow::Error;

    fn try_from(value: &sch::CharItemInfo) -> Result<Self, Self::Error> {
        let mut flags = EquipItemFlags::empty();

        if value.cash.into_bool() {
            flags.insert(EquipItemFlags::CASH);
        }

        if value.account_sharable.into_bool() {
            flags.insert(EquipItemFlags::ACCOUNT_SHARABLE);
        }

        if !value.drop_block.into_bool() {
            flags.insert(EquipItemFlags::CAN_DROP);
        }

        if value.equip_trade_block.into_bool() {
            flags.insert(EquipItemFlags::ON_EQUIP_TRADE_BLOCK);
        }

        if value.expire_on_logout.into_bool() {
            flags.insert(EquipItemFlags::ON_LOGOUT_EXPIRE);
        }

        if value.hide.into_bool() {
            flags.insert(EquipItemFlags::HIDE);
        }

        if value.name_tag.into_bool() {
            flags.insert(EquipItemFlags::NAME_TAG_ABLE);
        }

        if !value.not_sale.into_bool() {
            flags.insert(EquipItemFlags::SELL_ABLE);
        }

        if value.only.into_bool() || value.only_equip.into_bool() {
            flags.insert(EquipItemFlags::UNIQUE);
        }

        if value.quest.into_bool() {
            flags.insert(EquipItemFlags::QUEST_ITEM);
        }

        if value.trade_block.into_bool() || value.trad_block.into_bool() {
            flags.insert(EquipItemFlags::CAN_TRADE);
        }

        if !value.not_extend.into_bool() {
            flags.insert(EquipItemFlags::EXTEND_ABLE);
        }

        if value.time_limited.into_bool() {
            flags.insert(EquipItemFlags::IS_TIME_LIMITED);
        }

        Ok(flags)
    }
}

impl TryFrom<&sch::CharItemInfo> for EquipReq {
    type Error = anyhow::Error;

    fn try_from(value: &sch::CharItemInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            level: CharLevel(value.req_level.into_num() as u8),
            pop: Pop(value.req_pop.into_num() as i16),
            str: ItemStat(value.req_str.into_num() as u16),
            dex: ItemStat(value.req_dex.into_num() as u16),
            int: ItemStat(value.req_int.into_num() as u16),
            luk: ItemStat(value.req_luk.into_num() as u16),
            job: JobFlag::from(value.req_job.into_num() as i8),
            mob_level: 0,
        })
    }
}

pub struct EquipWithId<'a>(pub ItemId, pub &'a sch::CharItem);

impl<'a> TryFrom<&'a CharItemInfoAddition> for EquipAdditions {
    type Error = anyhow::Error;

    fn try_from(value: &'a CharItemInfoAddition) -> Result<Self, Self::Error> {
        let crit = value.critical.as_ref().map(|v| Addition {
            cond: EquipAdditionCond {
                job: v
                    .con
                    .as_ref()
                    .and_then(|v| v.job)
                    .map(|v| JobId::try_from(v as u16).unwrap()),
                level: CharLevel(v.con.as_ref().and_then(|v| v.lv.or(v.level)).unwrap_or(0) as u8),
                ..Default::default()
            },
            value: AdditionCrit {
                prob: ProcChance(v.prob.unwrap_or(0) as u8),
                damage: v.damage.into_num() as u8,
            },
        });

        //TODO
        Ok(Self {
            crit,
            elem_boost: None,
            boss: None,
            hp_mp_change: None,
            mob_category: None,
            mob_die: None,
            skill: None,
        })
    }
}

impl<'a> TryFrom<EquipWithId<'a>> for EquipItemTmpl {
    type Error = anyhow::Error;

    fn try_from(value: EquipWithId<'a>) -> Result<Self, Self::Error> {
        let EquipWithId(id, v) = value;

        let info = v.info.as_ref().unwrap();

        Ok(Self {
            id,
            flags: EquipItemFlags::try_from(info)?,
            req: EquipReq::try_from(info)?,
            stats: EquipStats::try_from(info)?,
            price: info.price.into_num() as Money,
            chat_balloon_id: info.chat_balloon.map(|v| v as u8),
            upgrade_slots: info.tuc.into_num() as u8,
            set_id: info.set_item_id.map(|v| SetId(v as u8)),
            enchant_category: EnchantCategory::try_from(info.enchant_category.into_num() as u8)?,
            max_enhancements: info.iuc_max.map(|v| v as u8),
            additions: info
                .addition
                .as_ref()
                .map(|v| EquipAdditions::try_from(v).unwrap()),
        })
    }
}

impl<'a> TryFrom<EquipWithId<'a>> for WeaponItemTmpl {
    type Error = anyhow::Error;

    fn try_from(value: EquipWithId<'a>) -> Result<Self, Self::Error> {
        let EquipWithId(id, v) = value;

        let info = v.info.as_ref().unwrap();

        let map_incm = |v: &Option<i64>| ItemStatRatio(v.unwrap_or(100) as u16);

        let equip_increase_magic_elem = EquipIncreaseMagicElems::from_fn(|elem| match elem {
            EquipIncreaseMagicElem::Fire => map_incm(&info.inc_rmaf),
            EquipIncreaseMagicElem::Ice => map_incm(&info.inc_rmai),
            EquipIncreaseMagicElem::Lightning => map_incm(&info.inc_rmal),
            EquipIncreaseMagicElem::Poison => map_incm(&info.inc_rmas),
        });

        Ok(Self {
            equip: value.try_into()?,
            atk_speed_degree: AttackSpeedDegree::try_from(info.attack_speed.into_num() as u8)?,
            attack_action: AttackAction::try_from(info.attack.into_num() as u8)?,
            equip_increase_magic_elem,
        })
    }
}

fn scroll_chance(v: &Option<StrOrInt>, d: &[(usize, i64)]) -> ScrollChance {
    match v {
        Some(v) => ScrollChance::Fixed(ProcChance(v.into_num() as u8)),
        None if d.is_empty() => ScrollChance::Perfect,
        _ => ScrollChance::Dynamic(d.iter().map(|(i, v)| ProcChance(*v as u8)).collect()),
    }
}

fn rand_range(value: &ItemValueInfo) -> Option<RandStatRange> {
    if value.randstat.into_num() == 1 {
        if value.inc_rand_vol.into_bool() {
            Some((-10..=10))
        } else {
            Some((-5..=5))
        }
    } else {
        None
    }
}

impl<'a> TryFrom<&'a ItemValueInfo> for ItemInfo {
    type Error = anyhow::Error;

    fn try_from(value: &'a ItemValueInfo) -> Result<Self, Self::Error> {
        let mut flags = ItemFlags::empty();
        if value.cash.into_bool() {
            flags.insert(ItemFlags::CASH);
        }
        if value.account_sharable.into_bool() {
            flags.insert(ItemFlags::ACCOUNT_SHARABLE);
        }
        if value.expire_on_logout.into_bool() {
            flags.insert(ItemFlags::EXPIRE_ON_LOGOUT);
        }
        if !value.not_sale.into_bool() {
            flags.insert(ItemFlags::SELL_ABLE);
        }
        if value.only.into_bool() {
            flags.insert(ItemFlags::UNIQUE);
        }
        if !value.not_extend.into_bool() {
            flags.insert(ItemFlags::EXTEND_ABLE);
        }
        if value.quest.into_bool() {
            flags.insert(ItemFlags::QUEST_ITEM);
        }
        if value.pquest.into_bool() {
            flags.insert(ItemFlags::PARTY_QUEST_ITEM);
        }
        //TODO?
        if !value.trade_block.into_bool() {
            flags.insert(ItemFlags::CAN_TRADE);
        }
        if value.time_limited.into_bool() {
            flags.insert(ItemFlags::IS_TIME_LIMITED);
        }

        Ok(ItemInfo {
            id: ItemId(0),
            flag: flags,
            slot_max: value
                .slot_max
                .as_ref()
                .map(|v| v.into_num() as u16)
                .unwrap_or(1),
            price: value.price.into_num() as Money,
            unit_price: value.unit_price.map(|v| v as f32),
            req: ItemReqInfo {
                level: value.req_level.map(|v| CharLevel(v.into_num() as u8)),
                fields: value
                    .req_map
                    .values()
                    .map(|v| FieldId(v.as_i64().unwrap() as u32))
                    .collect(),
            },
            pad: value.inc_pad.map(|v| ItemStat(v.into_num() as u16)),
            applyable_karma: value.karma.map(|v| v.into_num() as u8),
            max: value.max.into_num() as u16,
            req_quest_on_progress: value.req_quest_on_progress.map(|v| QuestId(v as u16)),
        })
    }
}

impl<'a> TryFrom<&'a ItemValueInfo> for ScrollItem {
    type Error = anyhow::Error;

    fn try_from(value: &'a ItemValueInfo) -> Result<Self, Self::Error> {
        let stats = map_base_stats!(value, inc_craft);

        Ok(Self {
            inc_stats: stats,
            inc_period: value.inc_period.map(|v| Duration::from_secs(v as u64)),
            prevent_slip: value.preventslip.into_bool(),
            warm_support: value.warmsupport.into_bool(),
            recover_slots: value.recover.into_num() as usize,
            item_enchant_category: EnchantCategory::try_from(
                value.enchant_category.into_num() as u8
            )?,
            rand_stats: rand_range(value),
            success: scroll_chance(&value.success, &map_to_list(&value.success_rates)),
            destroy: scroll_chance(
                &value.cursed.map(|v| StrOrInt::Variant1(v)),
                &map_to_list(&value.cursed_rates),
            ),
        })
    }
}

impl<'a> TryFrom<&'a ItemValueInfo> for MasteryBookItem {
    type Error = anyhow::Error;

    fn try_from(value: &'a ItemValueInfo) -> Result<Self, Self::Error> {
        Ok(MasteryBookItem {
            skills: value
                .skill
                .values()
                .cloned()
                .map(|v| SkillId(v as u32))
                .collect(),
            master_level: value.master_level.into_num() as SkillLevel,
            chance: ProcChance(value.success.into_num() as u8),
            required_skill_level: value.req_skill_level.into_num() as SkillLevel,
        })
    }
}

impl<'a> TryFrom<&'a ItemValue> for ConsumableItem {
    type Error = anyhow::Error;

    fn try_from(value: &'a ItemValue) -> Result<Self, Self::Error> {
        let spec = value.spec.as_ref().unwrap();

        let mut cure = CureFlags::empty();
        if spec.seal.into_bool() {
            cure.insert(CureFlags::SEAL);
        }
        if spec.curse.into_bool() {
            cure.insert(CureFlags::CURSE);
        }
        if spec.poison.into_bool() {
            cure.insert(CureFlags::POISON);
        }
        if spec.weakness.into_bool() {
            cure.insert(CureFlags::WEAKNESS);
        }
        if spec.darkness.into_bool() {
            cure.insert(CureFlags::DARKNESS);
        }

        Ok(ConsumableItem {
            hp: ItemStat(spec.hp.into_num() as u16),
            mp: ItemStat(spec.mp.into_num() as u16),
            hp_ratio: ItemStatRatio(spec.hp_r.into_num() as u16),
            mp_ratio: ItemStatRatio(spec.mp_r.into_num() as u16),
            exp: spec.expinc.or(spec.exp).into_num() as u32,
            max_hp_ratio: ItemStatRatio(spec.mhp_r.into_num() as u16),
            max_mp_ratio: ItemStatRatio(spec.mmp_r.into_num() as u16),
            acc: ItemStat(spec.acc.into_num() as u16),
            acc_ratio: ItemStatRatio(spec.acc_rate.into_num() as u16),
            eva: ItemStat(spec.eva.into_num() as u16),
            eva_ratio: ItemStatRatio(spec.eva_rate.into_num() as u16),
            speed: ItemStat(spec.speed.into_num() as u16),
            jump: ItemStat(spec.jump.into_num() as u16),
            speed_ratio: ItemStatRatio(spec.speed_rate.into_num() as u16),
            move_to: spec.move_to.map(|v| FieldId(v as u32)),
            ignore_continent: spec.ignore_continent.into_bool(),
            cp: spec.cp.as_ref().map(|v| *v as u32),
            cp_skill: spec.buff_skill.map(|v| SkillId(v as u32)),
            cure,
            attack_ix: spec.attack_index.map(|v| v as u8),
            barrier: spec.barrier.into_bool(),
            berserk: spec.berserk.map(|v| v as u32),
            bf_skill: spec.bf_skill.map(|v| v as u8),
            dojang_shield: spec.dojangshield.map(|v| v as u8),
            meso_up_by_item: spec.mesoupbyitem.into_bool(),
            item_up_by_item: spec.itemupbyitem.into_num() as u8,
            exp_buff: ItemStatRatio(spec.exp_buff.into_num() as u16),
            script: spec.script.clone(),
            time: spec.time.map(|v| Duration::from_secs(v as u64)),
            morph: spec.morph.as_ref().map(|v| *v as u8),
        })
    }
}

impl<'a> TryFrom<&'a ItemValue> for StateChangeItem {
    type Error = anyhow::Error;

    fn try_from(value: &'a ItemValue) -> Result<Self, Self::Error> {
        let spec = value.spec.as_ref().unwrap();

        let mut cure = CureFlags::empty();
        if spec.seal.into_bool() {
            cure.insert(CureFlags::SEAL);
        }
        if spec.curse.into_bool() {
            cure.insert(CureFlags::CURSE);
        }
        if spec.poison.into_bool() {
            cure.insert(CureFlags::POISON);
        }
        if spec.weakness.into_bool() {
            cure.insert(CureFlags::WEAKNESS);
        }
        if spec.darkness.into_bool() {
            cure.insert(CureFlags::DARKNESS);
        }

        Ok(StateChangeItem {
            hp: ItemStat(spec.hp.into_num() as u16),
            mp: ItemStat(spec.mp.into_num() as u16),
            max_hp_ratio: ItemStatRatio(spec.hp_r.into_num() as u16),
            max_mp_ratio: ItemStatRatio(spec.mp_r.into_num() as u16),
            exp: spec.expinc.or(spec.exp).into_num() as u32,
            acc: ItemStat(spec.acc.into_num() as u16),
            acc_ratio: ItemStatRatio(spec.acc_rate.into_num() as u16),
            eva: ItemStat(spec.eva.into_num() as u16),
            eva_ratio: ItemStatRatio(spec.eva_rate.into_num() as u16),
            speed: ItemStat(spec.speed.into_num() as u16),
            jump: ItemStat(spec.jump.into_num() as u16),
            speed_ratio: ItemStatRatio(spec.speed_rate.into_num() as u16),
            move_to: spec.move_to.map(|v| FieldId(v as u32)),
            ignore_continent: spec.ignore_continent.into_bool(),
            cp: spec.cp.as_ref().map(|v| *v as u32),
            cp_skill: spec.buff_skill.map(|v| SkillId(v as u32)),
            cure,
            attack_ix: spec.attack_index.map(|v| v as u8),
            barrier: spec.barrier.into_bool(),
            berserk: spec.berserk.map(|v| v as u32),
            bf_skill: spec.bf_skill.map(|v| v as u8),
            dojang_shield: spec.dojangshield.map(|v| v as u8),
            meso_up_by_item: spec.mesoupbyitem.into_bool(),
            item_up_by_item: spec.itemupbyitem.into_num() as u8,
            exp_buff: ItemStatRatio(spec.exp_buff.into_num() as u16),
            script: spec.script.clone(),
            time: spec.time.map(|v| Duration::from_millis(v as u64)),
            morph: spec.morph.as_ref().map(|v| *v as u8),
            pad: item_stat(&spec.pad),
            pdd: item_stat(&spec.pdd),
            mad: item_stat(&spec.mad),
            mdd: item_stat(&spec.mdd),
            craft: item_stat(&0),
            thaw: item_stat(&spec.thaw),
            tamed_mob_fatigue: spec.inc_fatigue.map(|v| v as i16),
            booster: spec.booster.map(|v| v as i16),
            party: spec.party.into_bool(),
        })
    }
}

pub struct ItemWithId<'a>(pub ItemId, pub &'a ItemValue);

impl<'a> TryFrom<ItemWithId<'a>> for BundleItemTmpl {
    type Error = anyhow::Error;

    fn try_from(value: ItemWithId<'a>) -> Result<Self, Self::Error> {
        let ItemWithId(id, v) = value;

        let info = v.info.as_ref().unwrap();
        let mut item_info: ItemInfo = info.try_into()?;
        item_info.id = id;

        let value = match info {
            _ if info.monster_book.into_bool() => BundleItemValue::MonsterBook(MonsterBookItem {
                mob: MobId(info.mob.into_num() as u32),
            }),
            _ if id.is_upgrade() => BundleItemValue::Scroll(ScrollItem::try_from(info)?),
            //TODO 203 field scroll
            _ if info.master_level.is_some() => {
                BundleItemValue::MasteryBook(MasteryBookItem::try_from(info)?)
            }
            _ if id.is_arrow_for_bow() || id.is_arrow_for_crossbow() || id.is_rechargable() => {
                BundleItemValue::Bullet
            }
            _ if id.is_summon_sack() => BundleItemValue::SummonSack,
            _ if id.is_state_change() => {
                BundleItemValue::StateChange(StateChangeItem::try_from(v)?)
            }
            //TODO
            _ if matches!(
                id.0 / 10_000,
                216 | 219 | 224 | 227 | 228 | 231 | 232 | 234 | 246 | 250
            ) =>
            {
                BundleItemValue::SummonSack
            }
            _ if id.is_consumable() => BundleItemValue::Consumable(ConsumableItem::try_from(v)?),
            _ if id.item_type() == ItemType::Etc => BundleItemValue::Etc,
            _ if id.item_type() == ItemType::Install => BundleItemValue::Install,
            _ if id.item_type() == ItemType::Cash => BundleItemValue::Cash,
            _ => todo!("id: {}", id),
        };

        Ok(Self {
            id,
            info: item_info,
            value,
        })
    }
}

impl<'a> TryFrom<&'a sch::ItemOption> for ItemOptionLevel {
    type Error = anyhow::Error;

    fn try_from(value: &'a sch::ItemOption) -> Result<Self, Self::Error> {
        let stats = map_base_stats!(value, inc_cr);
        let ratio_stats = map_base_ratio_stats!(value);

        Ok(Self {
            attack_type: value.attack_type.map(|v| v as u8),
            boss: value.boss.into_bool(),
            face: value.face.clone(),
            dam_reflect: item_stat(&value.da_mreflect),
            ignore_dam_ratio: item_stat_r(&value.ignore_da_mr),
            ignore_dam: item_stat(&value.ignore_dam),
            ignore_target_def: item_stat(&value.ignore_target_def),
            inc_all_skills: item_stat(&value.inc_allskill),
            meso_prop_ratio: item_stat_r(&value.inc_meso_prop),
            dam_ratio: item_stat_r(&value.inc_da_mr),
            stats: stats.0,
            ratio_stats,
            mp: item_stat(&value.mp),
            mp_restore: item_stat(&value.mp_restore),
            mpcon_reduce: item_stat(&value.mpcon_reduce),
            prop: value.prop.as_ref().map(|v| ProcChance(v.into_num() as u8)),
            recovery_hp: item_stat(&value.recovery_hp),
            recovery_mp: item_stat(&value.recovery_mp),
            recovery_up: item_stat(&value.recovery_up),
            level: value.level.into_num() as u8,
            time: value.time.map(|v| Duration::from_secs(v as u64)),
        })
    }
}

pub struct ItemOptWithId<'a>(pub ItemOptionId, pub &'a sch::ItemOptionsValue);

impl<'a> TryFrom<ItemOptWithId<'a>> for ItemOption {
    type Error = anyhow::Error;

    fn try_from(value: ItemOptWithId<'a>) -> Result<Self, Self::Error> {
        let ItemOptWithId(id, v) = value;

        let info = v.info.as_ref();
        let req_level = CharLevel(info.and_then(|v| v.req_level).unwrap_or(0) as u8);
        let ty = info.and_then(|v| v.option_type).unwrap_or(0) as u8;
        let levels = map_to_list(&v.level);
        Ok(Self {
            id,
            ty,
            req_level,
            levels: levels
                .iter()
                .map(|v| ItemOptionLevel::try_from(&v.1))
                .collect::<Result<_, _>>()?,
        })
    }
}
