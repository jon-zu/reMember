use std::sync::Arc;

use derive_more::{Deref, DerefMut, From};
use num_enum::TryFromPrimitive;
use shroom_meta::id::ItemId;
use shroom_meta::item::it::{PetItem, StackItem};
use shroom_meta::{id::item_id::InventoryType, item::it::EquipItem};
use shroom_proto95::shared::inventory::CharEquipSlot;

use shroom_srv::game::{
    inventory::{InvEventHandler, InvItem, InvSlotIndex, Inventory, NoopEventHandler},
    stack_inv::{InvStackItem, StackInvEventHandler, StackInventory},
};

use crate::services::{character::ItemStarterSet, item::SharedItemSvc};

pub const EQUIPPED_CAP: usize = 96;
pub const INV_ITEM_CAP: usize = 180;

#[derive(Debug, Deref, DerefMut, From)]
pub struct StackItemSlot {
    pub item: Box<StackItem>,
}

impl From<StackItem> for StackItemSlot {
    fn from(value: StackItem) -> Self {
        Self {
            item: Box::new(value),
        }
    }
}

impl InvItem for StackItemSlot {
    type Id = ItemId;
    type SlotIndex = usize;

    fn id(&self) -> Self::Id {
        self.info.item_id
    }
}

impl InvStackItem for StackItemSlot {
    fn max_stack_size(&self) -> usize {
        1024
    }

    fn quantity(&self) -> usize {
        self.quantity as usize
    }

    fn set_quantity(&mut self, count: usize) {
        self.quantity = count as u16;
        self.last_update += 1;
    }
}

#[derive(Debug, Deref, DerefMut, From)]
pub struct PetItemSlot {
    pub item: Box<PetItem>,
}

impl From<PetItem> for PetItemSlot {
    fn from(value: PetItem) -> Self {
        Self {
            item: Box::new(value),
        }
    }
}

impl InvItem for PetItemSlot {
    type Id = ItemId;
    type SlotIndex = usize;

    fn id(&self) -> Self::Id {
        self.info.item_id
    }
}

impl InvStackItem for PetItemSlot {
    fn max_stack_size(&self) -> usize {
        1
    }

    fn quantity(&self) -> usize {
        1
    }

    fn set_quantity(&mut self, _count: usize) {
        unreachable!()
    }
}

#[derive(Debug)]
pub struct EquipItemSlot {
    pub item_id: ItemId,
    pub item: Box<EquipItem>,
}

impl From<EquipItem> for EquipItemSlot {
    fn from(value: EquipItem) -> Self {
        Self {
            item_id: value.item_id,
            item: Box::new(value),
        }
    }
}

impl InvItem for EquipItemSlot {
    type Id = ItemId;
    type SlotIndex = usize;

    fn id(&self) -> Self::Id {
        self.item_id
    }
}

#[derive(Debug)]
pub struct EquippedItemSlot(pub EquipItemSlot);

impl From<EquipItem> for EquippedItemSlot {
    fn from(value: EquipItem) -> Self {
        Self(value.into())
    }
}

impl From<EquipItemSlot> for EquippedItemSlot {
    fn from(value: EquipItemSlot) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EquipSlot(pub CharEquipSlot);

impl From<CharEquipSlot> for EquipSlot {
    fn from(value: CharEquipSlot) -> Self {
        Self(value)
    }
}

impl From<EquipSlot> for CharEquipSlot {
    fn from(value: EquipSlot) -> Self {
        value.0
    }
}

impl InvSlotIndex for EquipSlot {
    fn to_ix(&self) -> usize {
        self.0 as usize
    }

    fn from_ix(slot: usize) -> Self {
        Self(CharEquipSlot::try_from_primitive(slot as u8).unwrap())
    }
}

impl InvItem for EquippedItemSlot {
    type Id = ItemId;
    type SlotIndex = EquipSlot;
    fn id(&self) -> Self::Id {
        self.0.item_id
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InventorySlot {
    Slot(InventoryType, u16),
    EquippedSlot(CharEquipSlot),
}

impl InventorySlot {
    pub fn as_slot_index(&self) -> u16 {
        match self {
            Self::Slot(_, slot) => *slot + 1,
            Self::EquippedSlot(slot) => (-(*slot as i16)) as u16,
        }
    }

    pub fn as_slot(&self) -> usize {
        match self {
            Self::Slot(_, v) => *v as usize,
            Self::EquippedSlot(v) => *v as usize,
        }
    }

    pub fn inv_type(&self) -> InventoryType {
        match self {
            Self::Slot(ty, _) => *ty,
            Self::EquippedSlot(_) => InventoryType::Equip,
        }
    }
}

impl TryFrom<(InventoryType, i16)> for InventorySlot {
    type Error = anyhow::Error;

    fn try_from((ty, slot): (InventoryType, i16)) -> Result<Self, Self::Error> {
        // TODO: need to work around the multiple equipped invs
        Ok(if ty.is_equip() && slot < 0 {
            Self::EquippedSlot(CharEquipSlot::try_from(-slot as u8)?)
        } else {
            if slot < 1 || slot > INV_ITEM_CAP as i16 {
                anyhow::bail!("Invalid slot: {slot}");
            }
            Self::Slot(ty, slot as u16 - 1)
        })
    }
}

#[derive(Debug)]
pub enum CashItemSlot {
    Stack(StackItemSlot),
    Pet(PetItemSlot),
}

impl CashItemSlot {
    pub fn as_pet(&self) -> Option<&PetItemSlot> {
        match self {
            Self::Pet(pet) => Some(pet),
            _ => None,
        }
    }

    pub fn as_stack(&self) -> Option<&StackItemSlot> {
        match self {
            Self::Stack(stack) => Some(stack),
            _ => None,
        }
    }
}

impl InvItem for CashItemSlot {
    type Id = ItemId;
    type SlotIndex = usize;

    fn id(&self) -> Self::Id {
        match self {
            Self::Stack(item) => item.info.item_id,
            Self::Pet(item) => item.info.item_id
        }
    }
}

impl InvStackItem for CashItemSlot {
    fn max_stack_size(&self) -> usize {
        match self {
            Self::Stack(item) => item.max_stack_size(),
            Self::Pet(_) => 1,
        }
    }

    fn quantity(&self) -> usize {
        match self {
            Self::Stack(item) => item.quantity(),
            Self::Pet(_) => 1,
        }
    }

    fn set_quantity(&mut self, count: usize) {
        match self {
            Self::Stack(item) => item.set_quantity(count),
            Self::Pet(_) => unreachable!(),
        }
    }
}
pub type EquippedInventory = Inventory<EquippedItemSlot, NoopEventHandler<EquippedItemSlot>>;
pub type EquipInventory = Inventory<EquipItemSlot, NoopEventHandler<EquipItemSlot>>;
pub type StackInv<H> = StackInventory<StackItemSlot, H>;
pub type CashInv<H> = StackInventory<CashItemSlot, H>;

pub type StackSlot = usize;

pub trait InvSetHandler {
    type StackTyHandler: StackInvEventHandler<Item = StackItemSlot>;
    type CashHandler: StackInvEventHandler<Item = CashItemSlot>;
    type EquipHandler: InvEventHandler<Item = EquipItemSlot>;

    fn create_eq_handler(svc: Arc<SharedItemSvc>) -> Self::EquipHandler;
    fn create_stack_handler(svc: Arc<SharedItemSvc>, ty: InventoryType) -> Self::StackTyHandler;
    fn create_cash_handler(svc: Arc<SharedItemSvc>) -> Self::CashHandler;
}

pub struct NoopInvSetHandler;

pub struct NoopStackInvTyHandler(pub Arc<SharedItemSvc>);

impl InvEventHandler for NoopStackInvTyHandler {
    type Item = StackItemSlot;

    fn on_add(&mut self, _item: &Self::Item, _slot: <Self::Item as InvItem>::SlotIndex) {}

    fn on_remove(&mut self, _item: &Self::Item, _slot: <Self::Item as InvItem>::SlotIndex) {}

    fn on_update(&mut self, _item: &Self::Item, _slot: <Self::Item as InvItem>::SlotIndex) {}

    fn on_swap(
        &mut self,
        _slot_a: <Self::Item as InvItem>::SlotIndex,
        _slot_b: <Self::Item as InvItem>::SlotIndex,
    ) {
    }
}

impl StackInvEventHandler for NoopStackInvTyHandler {
    fn on_quantity_change(
        &mut self,
        _item: &Self::Item,
        _slot: <Self::Item as InvItem>::SlotIndex,
    ) {
    }

    fn new_stack(&mut self, id: <Self::Item as InvItem>::Id, quantity: usize) -> Self::Item {
        self.0.new_stack_item(id, quantity).into()
    }
}

pub struct NoopCashInvHandler(Arc<SharedItemSvc>);

impl InvEventHandler for NoopCashInvHandler {
    type Item = CashItemSlot;

    fn on_add(&mut self, _item: &Self::Item, _slot: <Self::Item as InvItem>::SlotIndex) {}

    fn on_remove(&mut self, _item: &Self::Item, _slot: <Self::Item as InvItem>::SlotIndex) {}

    fn on_update(&mut self, _item: &Self::Item, _slot: <Self::Item as InvItem>::SlotIndex) {}

    fn on_swap(
        &mut self,
        _slot_a: <Self::Item as InvItem>::SlotIndex,
        _slot_b: <Self::Item as InvItem>::SlotIndex,
    ) {
    }
}

impl StackInvEventHandler for NoopCashInvHandler {
    fn on_quantity_change(
        &mut self,
        _item: &Self::Item,
        _slot: <Self::Item as InvItem>::SlotIndex,
    ) {
    }

    fn new_stack(&mut self, id: <Self::Item as InvItem>::Id, quantity: usize) -> Self::Item {
        CashItemSlot::Stack(self.0.new_stack_item(id, quantity).into())
    }
}

impl InvSetHandler for NoopInvSetHandler {
    type StackTyHandler = NoopStackInvTyHandler;
    type CashHandler = NoopCashInvHandler;
    type EquipHandler = NoopEventHandler<EquipItemSlot>;

    fn create_eq_handler(_svc: Arc<SharedItemSvc>) -> Self::EquipHandler {
        NoopEventHandler::default()
    }

    fn create_stack_handler(svc: Arc<SharedItemSvc>, _ty: InventoryType) -> Self::StackTyHandler {
        NoopStackInvTyHandler(svc)
    }

    fn create_cash_handler(svc: Arc<SharedItemSvc>) -> Self::CashHandler {
        NoopCashInvHandler(svc)
    }
}

#[derive(Debug)]
pub struct InventorySet<T: InvSetHandler> {
    pub equipped: EquippedInventory,
    pub masked_equipped: EquippedInventory,
    pub equip: EquipInventory,
    pub consume: StackInv<T::StackTyHandler>,
    pub misc: StackInv<T::StackTyHandler>,
    pub etc: StackInv<T::StackTyHandler>,
    pub cash: CashInv<T::CashHandler>,
    item_svc: Arc<SharedItemSvc>,
}

impl<T: InvSetHandler> InventorySet<T> {
    pub fn with_default_slots(svc: Arc<SharedItemSvc>) -> Self {
        const DEFAULT_SLOTS: usize = 48;

        Self {
            equipped: EquippedInventory::new(NoopEventHandler::default(), EQUIPPED_CAP),
            masked_equipped: EquippedInventory::new(NoopEventHandler::default(), EQUIPPED_CAP),
            equip: EquipInventory::new(NoopEventHandler::default(), DEFAULT_SLOTS),
            consume: StackInv::new(
                T::create_stack_handler(svc.clone(), InventoryType::Consume),
                DEFAULT_SLOTS,
            ),
            misc: StackInv::new(
                T::create_stack_handler(svc.clone(), InventoryType::Install),
                DEFAULT_SLOTS,
            ),
            etc: StackInv::new(
                T::create_stack_handler(svc.clone(), InventoryType::Etc),
                DEFAULT_SLOTS,
            ),
            cash: CashInv::new(T::create_cash_handler(svc.clone()), DEFAULT_SLOTS),
            item_svc: svc,
        }
    }

    pub fn with_handler<U: InvSetHandler>(self, svc: Arc<SharedItemSvc>) -> InventorySet<U> {
        InventorySet {
            equipped: self.equipped,
            masked_equipped: self.masked_equipped,
            equip: self.equip,
            consume: self
                .consume
                .with_handler(U::create_stack_handler(svc.clone(), InventoryType::Consume)),
            misc: self
                .misc
                .with_handler(U::create_stack_handler(svc.clone(), InventoryType::Install)),
            etc: self
                .etc
                .with_handler(U::create_stack_handler(svc.clone(), InventoryType::Etc)),
            cash: self.cash.with_handler(U::create_cash_handler(svc.clone())),
            item_svc: svc,
        }
    }

    pub fn fill_with_starter_set(&mut self, set: ItemStarterSet) -> anyhow::Result<()> {
        self.equipped.set(
            CharEquipSlot::Top.into(),
            self.item_svc.create_equip(set.top)?.into(),
        )?;
        self.equipped.set(
            CharEquipSlot::Bottom.into(),
            self.item_svc.create_equip(set.bottom)?.into(),
        )?;
        self.equipped.set(
            CharEquipSlot::Weapon.into(),
            self.item_svc.create_equip(set.weapon)?.into(),
        )?;
        self.equipped.set(
            CharEquipSlot::Shoes.into(),
            self.item_svc.create_equip(set.shoes)?.into(),
        )?;

        self.etc
            .try_add(self.item_svc.new_stack_item(set.guide, 1).into())?;

        Ok(())
    }

    pub fn get_stack_inventory_mut(
        &mut self,
        ty: InventoryType,
    ) -> anyhow::Result<&mut StackInv<T::StackTyHandler>> {
        Ok(match ty {
            InventoryType::Consume => &mut self.consume,
            InventoryType::Install => &mut self.misc,
            InventoryType::Etc => &mut self.etc,
            _ => anyhow::bail!("Invalid stack inventory"),
        })
    }

    pub fn get_stack_inventory(
        &self,
        ty: InventoryType,
    ) -> anyhow::Result<&StackInv<T::StackTyHandler>> {
        Ok(match ty {
            InventoryType::Consume => &self.consume,
            InventoryType::Install => &self.misc,
            InventoryType::Etc => &self.etc,
            _ => anyhow::bail!("Invalid stack inventory"),
        })
    }

    pub fn get_equipped_inventory_mut(
        &mut self,
        ty: InventoryType,
    ) -> anyhow::Result<&mut EquippedInventory> {
        Ok(match ty {
            InventoryType::Equipped => &mut self.equipped,
            InventoryType::Equip => &mut self.equipped,
            _ => anyhow::bail!("Invalid equipped inventory"),
        })
    }

    pub fn get_equipped_inventory(&self, ty: InventoryType) -> anyhow::Result<&EquippedInventory> {
        Ok(match ty {
            InventoryType::Equipped => &self.equipped,
            InventoryType::Equip => &self.equipped,
            _ => anyhow::bail!("Invalid equipped inventory"),
        })
    }

    pub fn get_cash_inventory(&self) -> &CashInv<T::CashHandler> {
        &self.cash
    }

    pub fn get_cash_inventory_mut(&mut self) -> &mut CashInv<T::CashHandler> {
        &mut self.cash
    }

    pub fn slots(&self, ty: InventoryType) -> usize {
        if ty.is_stack() {
            self.get_stack_inventory(ty).unwrap().capacity()
        } else if ty == InventoryType::Cash {
            self.get_cash_inventory().capacity()
        } else {
            self.get_equipped_inventory(ty).unwrap().capacity()
        }
    }
}
