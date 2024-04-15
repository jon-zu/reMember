use shroom_pkt::{
    mark_shroom_bitflags, CondOption, ShroomExpirationTime, ShroomOption8, ShroomPacket,
    ShroomPacketEnum, ShroomTime,
};

use shroom_meta::{
    id::{ItemId, ItemOptionId},
    item::{
        it::{EquipItem, PetItem, StackItem},
        EquipStat,
    },
};

use super::NameStr;

#[derive(Debug, ShroomPacket)]
pub struct ItemInfo {
    pub item_id: ItemId,
    pub cash_id: ShroomOption8<u64>,
    pub expiration: ShroomExpirationTime,
}

impl ItemInfo {
    pub fn is_rechargable(&self) -> bool {
        self.item_id.is_rechargable()
    }

    pub fn has_sn(&self) -> bool {
        self.cash_id.is_none()
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemFlags : u16 {
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const PreventColdness = 0x04;
        const Untradeable = 0x08;
        const ScissorsApplied = 0x10;
        const Sandbox = 0x40;
        const PetCome = 0x80;
        const AccountSharing = 0x100;
        const MergeUntradeable = 0x200;
    }
}
mark_shroom_bitflags!(ItemFlags);

impl From<shroom_meta::item::it::ItemFlags> for ItemFlags {
    fn from(value: shroom_meta::item::it::ItemFlags) -> Self {
        let mut flags = ItemFlags::empty();
        if value.contains(shroom_meta::item::it::ItemFlags::Protected) {
            flags |= ItemFlags::Protected;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::PreventSlipping) {
            flags |= ItemFlags::PreventSlipping;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::PreventColdness) {
            flags |= ItemFlags::PreventColdness;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::Untradeable) {
            flags |= ItemFlags::Untradeable;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::ScissorsApplied) {
            flags |= ItemFlags::ScissorsApplied;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::Sandbox) {
            flags |= ItemFlags::Sandbox;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::PetCome) {
            flags |= ItemFlags::PetCome;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::AccountSharing) {
            flags |= ItemFlags::AccountSharing;
        }
        if value.contains(shroom_meta::item::it::ItemFlags::MergeUntradeable) {
            flags |= ItemFlags::MergeUntradeable;
        }
        flags
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemBundleFlags : u16 {
        const Protected = 0x01;
        const TradingPossible = 0x02;
    }
}
mark_shroom_bitflags!(ItemBundleFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemPetFlags : u16 {
        const Protected = 0x01;
        const TradingPossible = 0x02;
    }
}
mark_shroom_bitflags!(ItemPetFlags);

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct ItemEquipFlags : u16 {
        const Protected = 0x01;
        const PreventSlipping = 0x02;
        const SupportWarm = 0x04;
        const Binded = 0x08;
        //const TradingPossible = 0x1;
    }
}
mark_shroom_bitflags!(ItemEquipFlags);

#[derive(Debug, ShroomPacket)]
pub struct PetItemInfo {
    pub pet_name: NameStr,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8,                     /* repleteness */
    pub expiration: ShroomExpirationTime, /* dateDead */
    pub pet_attr: u16,                    /* PetAttribute  pet is only loaded when attr == 1*/
    pub skill: u16,
    pub remain_life: u32,
    pub attr: ItemPetFlags,
}
#[derive(Debug, ShroomPacket)]
pub struct EquipStats {
    pub str: u16,
    pub dex: u16,
    pub int: u16,
    pub luk: u16,
    pub hp: u16,
    pub mp: u16,
    pub pad: u16,
    pub mad: u16,
    pub pdd: u16,
    pub mdd: u16,
    pub acc: u16,
    pub eva: u16,
    pub craft: u16,
    pub speed: u16,
    pub jump: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct EquipAllStats {
    pub remaining_upgrade_slots: u8,
    pub upgrade_count: u8,
    pub stats: EquipStats,
    pub title: String,
    pub flags: ItemFlags,
}

#[derive(Debug, ShroomPacket)]
pub struct ItemPetData {
    pub info: ItemInfo,
    pub name: NameStr,
    pub level: u8,
    pub tameness: u16,
    pub fullness: u8,
    pub dead_at: ShroomExpirationTime,
    pub attribute1: u16,
    pub skill: u16,
    pub remain_life: u32,
    pub attribute2: u16,
}

impl From<&PetItem> for ItemPetData {
    fn from(value: &PetItem) -> Self {
        Self {
            info: ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id().into(),
                expiration: value
                    .expiration
                    .map(|v| v.and_utc().try_into().unwrap())
                    .into(),
            },
            name: value.name.as_str().try_into().unwrap(),
            level: value.level,
            tameness: value.tameness,
            fullness: value.fullness,
            dead_at: value
                .dead_at
                .map(|v| v.and_utc().try_into().unwrap())
                .unwrap_or(ShroomTime::now().into()),
            attribute1: value.attr1,
            skill: value.skill,
            remain_life: value.remaining_life,
            attribute2: value.attr2,
        }
    }
}

#[derive(Debug, ShroomPacket)]
pub struct ItemStackData {
    pub info: ItemInfo,
    pub quantity: u16, /* nNumber */
    pub title: String,
    pub flag: ItemFlags,
    #[pkt(check(field = "info", cond = "ItemInfo::is_rechargable"))]
    pub sn: CondOption<u64>,
}

impl From<&StackItem> for ItemStackData {
    fn from(value: &StackItem) -> Self {
        Self {
            info: ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id().into(),
                expiration: value
                    .expiration
                    .map(|v| v.and_utc().try_into().unwrap())
                    .into(),
            },
            quantity: value.quantity,
            title: value.owner.clone().unwrap_or_default(),
            flag: value.flags.into(),
            sn: value
                .item_id
                .is_rechargable()
                .then(|| value.sn())
                .flatten()
                .into(),
        }
    }
}

#[derive(Debug, ShroomPacket)]
pub struct EquipItemInfo {
    pub info: ItemInfo,
    pub stats: EquipAllStats,
    pub lvl_up_ty: u8,
    pub lvl: u8,
    pub exp: u32,
    pub durability: i32,
    pub hammer_count: u32,
    pub grade: u8,
    pub stars: u8,
    pub options: [ItemOptionId; 3],
    pub sockets: [u16; 2],
    #[pkt(check(field = "info", cond = "ItemInfo::has_sn"))]
    pub sn: CondOption<u64>,
    pub equipped_at: ShroomTime,
    pub prev_bonus_exp_rate: i32,
}

impl From<&EquipItem> for EquipItemInfo {
    fn from(value: &EquipItem) -> Self {
        let s = &value.stats.0;
        let stats = EquipStats {
            str: s[EquipStat::Str].0,
            dex: s[EquipStat::Dex].0,
            int: s[EquipStat::Int].0,
            luk: s[EquipStat::Luk].0,
            hp: s[EquipStat::Hp].0,
            mp: s[EquipStat::Mp].0,
            pad: s[EquipStat::Pad].0,
            mad: s[EquipStat::Mad].0,
            pdd: s[EquipStat::Pdd].0,
            mdd: s[EquipStat::Mdd].0,
            acc: s[EquipStat::Acc].0,
            eva: s[EquipStat::Eva].0,
            craft: s[EquipStat::Craft].0,
            speed: s[EquipStat::Speed].0,
            jump: s[EquipStat::Jump].0,
        };

        Self {
            info: ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id().into(),
                expiration: value
                    .expiration
                    .map(|v| v.and_utc().try_into().unwrap())
                    .into(),
            },
            stats: EquipAllStats {
                remaining_upgrade_slots: value.upgrade_slots,
                upgrade_count: value.upgrades,
                stats,
                title: value.owner.clone().unwrap_or_default(),
                flags: value.flags.into(),
            },
            equipped_at: value
                .equipped_at
                .map(|v| v.and_utc().try_into().unwrap())
                .unwrap_or(ShroomTime::now()),
            lvl_up_ty: value
                .level_info
                .as_ref()
                .map(|l| l.item_level_ty)
                .unwrap_or(0),
            lvl: value.level_info.as_ref().map(|l| l.level).unwrap_or(0),
            exp: value.level_info.as_ref().map(|l| l.exp).unwrap_or(0),
            durability: value.durability.into(),
            hammer_count: value.hammers_used as u32,
            grade: value.grade as u8,
            stars: value.stars,
            options: value.options.0,
            sockets: value.sockets.0,
            sn: value.sn().into(),
            prev_bonus_exp_rate: -1,
        }
    }
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum Item {
    Equip(EquipItemInfo) = 1,
    Stack(ItemStackData) = 2,
    Pet(ItemPetData) = 3,
    Equipped(()) = 255,
}
