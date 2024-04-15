use shroom_pkt::{
    shroom_enum_code, time::Ticks, with_opcode, ShroomIndexListZ16, ShroomIndexListZ8, ShroomList8, ShroomPacket, ShroomPacketEnum, ShroomTime
};


use shroom_meta::id::{item_id::InventoryType, ItemId};
use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
};

use super::{char::InventorySize, item::Item};

//TODO indexing

shroom_enum_code!(
    CharEquipSlot,
    u8,
    Hair = 0,
    Hat = 1,
    FaceAccessory = 2,
    EyeAccessory = 3,
    EarAccessory = 4,
    Top = 5,
    Bottom = 6,
    Shoes = 7,
    Gloves = 8,
    Cape = 9,
    Shield = 10,
    Weapon = 11,
    Ring1 = 12,
    Ring2 = 13,
    PetEquip = 14,
    Ring3 = 15,
    Ring4 = 16,
    Pendant = 17,
    TamedMob = 18,
    Saddle = 19,
    MobEquip = 20,
    PetRingLabel = 21,
    PetItem = 0x16,
    PetMeso = 0x17,
    PetHpConsume = 0x18,
    PetMpConsume = 0x19,
    PetSweepForDrop = 0x1A,
    PetLongRange = 0x1B,
    PetPickUpOthers = 0x1C,
    PetRingQuote = 0x1D,
    Pet2Wear = 0x1E,
    Pet2Label = 0x1F,
    Pet2Quote = 0x20,
    Pet2Item = 0x21,
    Pet2Meso = 0x22,
    Pet2SweepForDrop = 0x23,
    Pet2LongRange = 0x24,
    Pet2PickUpOthers = 0x25,
    Pet3Wear = 0x26,
    Pet3Label = 0x27,
    Pet3Quote = 0x28,
    Pet3Item = 0x29,
    Pet3Meso = 0x2A,
    Pet3SweepForDrop = 0x2B,
    Pet3LongRange = 0x2C,
    Pet3PickUpOthers = 0x2D,
    Pet1IgnoreItems = 0x2E,
    Pet2IgnoreItems = 0x2F,
    Pet3IgnoreItems = 0x30,
    Medal = 0x31,
    Belt = 0x32,
    Shoulder = 0x33,
    Nothing3 = 0x36,
    Nothing2 = 0x37,
    Nothing1 = 0x38,
    Nothing0 = 0x39,
    ExtPendant1 = 0x3B,
    Ext1 = 0x3C,
    Ext2 = 0x3D,
    Ext3 = 0x3E,
    Ext4 = 0x3F,
    Ext5 = 0x40,
    Ext6 = 0x41,
    Sticker = 0x64
);

impl CharEquipSlot {
    pub fn can_swap(&self, other: &Self) -> bool {
        // TODO handle ring etc
        self == other
    }
}

shroom_enum_code!(
    CashEquippedSlot,
    u8,
    Hat = 101,
    Face = 102,
    Eye = 103,
    Top = 104,
    Overall = 105,
    Bottom = 106,
    Shoes = 107,
    Gloves = 108,
    Cape = 109,
    Shield = 110,
    Weapon = 111,
    Ring1 = 112,
    Ring2 = 113,
    // 14??
    Ring3 = 115,
    Ring4 = 116,
    Pendant = 117,
    TamedMob = 118
);

shroom_enum_code!(
    DragonEquipSlot,
    u16,
    Cap = 0x3e8,
    Pendant = 0x3e9,
    Wings = 0x3ea,
    Shoes = 0x3eb
);

shroom_enum_code!(
    MechanicEquipSlot,
    u16,
    Engine = 0x44C,
    Arm = 0x44D,
    Leg = 0x44E,
    Frame = 0x44F,
    Transitor = 0x450
);

#[derive(Debug, ShroomPacket)]
pub struct InventoryInfo {
    slot_limits: InventorySize,
    timestamp: ShroomTime,
    equipped: ShroomIndexListZ16<Item>,
    equipped_cash: ShroomIndexListZ16<Item>,
    equip: ShroomIndexListZ16<Item>,
    pad: u16,
    _use: ShroomIndexListZ8<Item>,
    setup: ShroomIndexListZ8<Item>,
    etc: ShroomIndexListZ8<Item>,
    cash: ShroomIndexListZ8<Item>,
}

#[derive(Debug, ShroomPacket)]
pub struct SortItemsPacket {
    timestamp: Ticks,
    inv_ty: InventoryType,
}

#[derive(Debug, ShroomPacket)]
pub struct MoveItemsPacket {
    timestamp: Ticks,
    inv_ty: InventoryType,
    slot: u16,
    action: u16,
    count: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct UseItemPacket {
    timestamp: Ticks,
    slot: u16,
    item_id: ItemId,
}


#[derive(Debug, ShroomPacket)]
pub struct InvOpAdd {
    pub inv_type: InventoryType,
    pub pos: u16,
    pub item: Item,
}

#[derive(Debug, ShroomPacket)]
pub struct InvOpUpdateQuantity {
    pub inv_type: InventoryType,
    pub pos: u16,
    pub quantity: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct InvOpMove {
    pub inv_type: InventoryType,
    pub pos: u16,
    pub new_pos: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct InvOpRemove {
    pub inv_type: InventoryType,
    pub pos: u16,
}

#[derive(Debug, ShroomPacket)]
pub struct InvOpUpdateExp {
    pub inv_type: InventoryType,
    pub pos: u16,
}

#[derive(Debug, ShroomPacketEnum)]
#[repr(u8)]
pub enum InventoryOperation {
    Add(InvOpAdd) = 0,
    UpdateQuantity(InvOpUpdateQuantity) = 1,
    Move(InvOpMove) = 2,
    Remove(InvOpRemove) = 3,
    UpdateExp(InvOpUpdateExp) = 4,
}

impl InventoryOperation {
    pub fn remove(inv: InventoryType, pos: u16) -> Self {
        Self::Remove(InvOpRemove { inv_type: inv, pos })
    }

    pub fn update_exp(inv: InventoryType, pos: u16) -> Self {
        Self::Remove(InvOpRemove { inv_type: inv, pos })
    }

    pub fn update_quantity(inv: InventoryType, pos: u16, quantity: u16) -> Self {
        Self::UpdateQuantity(InvOpUpdateQuantity {
            inv_type: inv,
            pos,
            quantity,
        })
    }

    pub fn add(inv: InventoryType, pos: u16, item: Item) -> Self {
        Self::Add(InvOpAdd {
            inv_type: inv,
            pos,
            item,
        })
    }

    pub fn mov(inv: InventoryType, src: u16, dst: u16) -> Self {
        Self::Move(InvOpMove {
            inv_type: inv,
            pos: src,
            new_pos: dst,
        })
    }
}

#[derive(Debug, ShroomPacket)]
pub struct InventoryOperationsResp {
    pub reset_excl: bool,
    pub operations: ShroomList8<InventoryOperation>,
    pub secondary_stat_changed: bool, //TODO optional tail byte
                                      // Updated when operation is done on equip inv, either Move(2), Remove(3)
}
with_opcode!(InventoryOperationsResp, SendOpcodes::InventoryOperation);

#[derive(ShroomPacket, Debug)]
pub struct InvGrowResp {
    pub inv_type: InventoryType, //TODO only first 6 inv can grow
    pub new_size: u8,
}
with_opcode!(InvGrowResp, SendOpcodes::InventoryGrow);

#[derive(ShroomPacket, Debug)]
pub struct InvChangeSlotPosReq {
    pub ticks: Ticks,
    pub inv_type: InventoryType,
    pub from: i16,
    pub to: i16,
    pub count: u16,
}
with_opcode!(
    InvChangeSlotPosReq,
    RecvOpcodes::UserChangeSlotPositionRequest
);

#[derive(ShroomPacket, Debug)]
pub struct InvSortRequest {
    pub ticks: Ticks,
    pub inv_type: InventoryType,
}
with_opcode!(InvSortRequest, RecvOpcodes::UserSortItemRequest);

// Use an item like magnifying glass, maybe hammer aswell?
#[derive(ShroomPacket, Debug)]
pub struct ItemReleaseReq {
    pub ticks: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
}
with_opcode!(ItemReleaseReq, RecvOpcodes::UserItemReleaseRequest);

#[derive(Debug, ShroomPacket)]
pub struct GatherItemReq {
    pub timestamp: Ticks,
    pub inv_ty: InventoryType,
}
with_opcode!(GatherItemReq, RecvOpcodes::UserGatherItemRequest);

#[derive(Debug, ShroomPacket)]
pub struct ItemOptionUpgradeReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
    pub enchant_skill: bool,
}
with_opcode!(
    ItemOptionUpgradeReq,
    RecvOpcodes::UserItemOptionUpgradeItemUseRequest
);


#[derive(Debug, ShroomPacket)]
pub struct ItemStatChangeItemUseReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub item_id: ItemId
}
with_opcode!(
    ItemStatChangeItemUseReq,
    RecvOpcodes::UserStatChangeItemUseRequest
);

#[derive(Debug, ShroomPacket)]
pub struct ItemHyperUpgradeReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
    pub enchant_skill: bool,
}
with_opcode!(
    ItemHyperUpgradeReq,
    RecvOpcodes::UserHyperUpgradeItemUseRequest
);

shroom_enum_code!(
    WhiteScrollFlag,
    u16,
    Unset = 1,
    Set = 2
);


impl From<WhiteScrollFlag> for bool {
    fn from(val: WhiteScrollFlag) -> Self {
        val == WhiteScrollFlag::Set
    }
}

#[derive(Debug, ShroomPacket)]
pub struct ItemUpgradeReq {
    pub timestamp: Ticks,
    pub use_slot: u16,
    pub equip_slot: u16,
    pub using_white_scroll: WhiteScrollFlag,
    pub enchant_skill: bool,
}
with_opcode!(ItemUpgradeReq, RecvOpcodes::UserUpgradeItemUseRequest);

#[derive(Debug, ShroomPacket)]
pub struct TamingMobUseFoodReq {
    pub timestamp: Ticks,
    pub food_slot: u16,
    pub item_id: ItemId,
}
with_opcode!(
    TamingMobUseFoodReq,
    RecvOpcodes::UserTamingMobFoodItemUseRequest
);

#[derive(Debug, ShroomPacket)]
pub struct ItemOpenUIReq {
    pub timestamp: Ticks,
    pub slot: u16,
    pub item_id: ItemId,
}
with_opcode!(ItemOpenUIReq, RecvOpcodes::UserUIOpenItemUseRequest);

#[derive(Debug, ShroomPacket)]
pub struct ItemLearnSkillReq {
    pub timestamp: Ticks,
    pub slot: u16,
    pub item_id: ItemId,
}
with_opcode!(ItemLearnSkillReq, RecvOpcodes::UserSkillLearnItemUseRequest);

#[derive(Debug, ShroomPacket)]
pub struct UserSitReq {
    pub seat_id: u16,
}

impl UserSitReq {
    pub fn get_up() -> Self {
        Self::seat(u16::MAX)
    }

    pub fn seat(seat_id: u16) -> Self {
        Self { seat_id }
    }
}
with_opcode!(UserSitReq, RecvOpcodes::UserSitRequest);
