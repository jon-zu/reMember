use std::{collections::VecDeque, sync::Arc};

use anyhow::Context;
use either::Either;

use rand::thread_rng;
use shroom_data::{
    model::inv::{
        CashItemSlot, EquipItemSlot, InvSetHandler, InventorySet, InventorySlot, NoopInvSetHandler,
        StackItemSlot,
    },
    services::item::{ItemService, SharedItemSvc},
};
use shroom_meta::{
    id::{item_id::InventoryType, ItemId},
    item::{
        it::{EquipItem, PetItem, StackItem, UpgradeResult},
        EquipBaseStats,
    },
    MetaService,
};
use shroom_pkt::ShroomIndexListZ8;
use shroom_proto95::shared::{
    char::InventorySize,
    inventory::{CharEquipSlot, InventoryOperation},
    item::Item,
};
use shroom_srv::game::stack_inv::InvStackItem;
use shroom_srv::game::{
    inventory::{InvError, InvEventHandler, InvItem, NoopEventHandler},
    stack_inv::StackInvEventHandler,
};

#[derive(Debug)]
pub struct DropStackItem(pub ItemId, pub usize);

#[derive(Debug, Default)]
struct PendingOperations(VecDeque<InventoryOperation>);

impl PendingOperations {
    fn add(&mut self, ty: InventoryType, item: Item, slot: u16) {
        self.0
            .push_back(InventoryOperation::add(ty, slot, item));
    }

    fn remove(&mut self, ty: InventoryType, slot: u16) {
        self.0.push_back(InventoryOperation::remove(ty, slot));
    }

    fn mov(&mut self, ty: InventoryType, src: u16, dst: u16) {
        self.0.push_back(InventoryOperation::mov(ty, src, dst));
    }

    fn update_quantity(&mut self, ty: InventoryType, slot: u16, quantity: u16) {
        self.0
            .push_back(InventoryOperation::update_quantity(ty, slot, quantity));
    }

    fn update(&mut self, ty: InventoryType, slot: u16, item: Item) {
        self.remove(ty, slot);
        self.add(ty, item, slot);
    }

    fn drain_into(&mut self, target: &mut Vec<InventoryOperation>) {
        target.extend(self.0.drain(..));
    }
}

#[derive(Debug)]
pub struct StackTyInvHandler {
    ops: PendingOperations,
    ty: InventoryType,
    item_svc: Arc<SharedItemSvc>,
}

impl InvEventHandler for StackTyInvHandler {
    type Item = StackItemSlot;

    fn on_add(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.ops
            .add(self.ty, Item::Stack(item.as_ref().into()), slot as u16 + 1)
    }

    fn on_remove(&mut self, _item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.ops.remove(self.ty, slot as u16 + 1)
    }

    fn on_update(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.on_remove(item, slot);
        self.on_add(item, slot);
    }

    fn on_swap(
        &mut self,
        slot_a: <Self::Item as InvItem>::SlotIndex,
        slot_b: <Self::Item as InvItem>::SlotIndex,
    ) {
        self.ops.mov(self.ty, slot_a as u16 + 1, slot_b as u16 + 1);
    }
}

impl StackInvEventHandler for StackTyInvHandler {
    fn on_quantity_change(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.ops
            .update_quantity(self.ty, slot as u16 + 1, item.quantity() as u16)
    }

    fn new_stack(&mut self, id: <Self::Item as InvItem>::Id, quantity: usize) -> Self::Item {
        self.item_svc.new_stack_item(id, quantity).into()
    }
}

#[derive(Debug)]
pub struct CashInvHandler {
    ops: PendingOperations,
    item_svc: Arc<SharedItemSvc>,
}

impl InvEventHandler for CashInvHandler {
    type Item = CashItemSlot;

    fn on_add(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.ops.add(
            InventoryType::Cash,
            match item {
                CashItemSlot::Stack(item) => Item::Stack(item.as_ref().into()),
                CashItemSlot::Pet(item) => Item::Pet(item.as_ref().into()),
            },
            slot as u16 + 1,
        );
    }

    fn on_remove(&mut self, _item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.ops.remove(InventoryType::Cash, slot as u16 + 1);
    }

    fn on_update(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        self.on_remove(item, slot);
        self.on_add(item, slot);
    }

    fn on_swap(
        &mut self,
        slot_a: <Self::Item as InvItem>::SlotIndex,
        slot_b: <Self::Item as InvItem>::SlotIndex,
    ) {
        self.ops
            .mov(InventoryType::Cash, slot_a as u16 + 1, slot_b as u16 + 1);
    }
}

impl StackInvEventHandler for CashInvHandler {
    fn on_quantity_change(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex) {
        if let CashItemSlot::Stack(ref item) = item {
            self.ops
                .update_quantity(InventoryType::Cash, slot as u16 + 1, item.quantity() as u16);
        }
    }

    fn new_stack(&mut self, id: <Self::Item as InvItem>::Id, quantity: usize) -> Self::Item {
        //TODO handle pets/cash unique items
        CashItemSlot::Stack(self.item_svc.new_stack_item(id, quantity).into())
    }
}

#[derive(Debug)]
pub struct CharInvHandler;

impl InvSetHandler for CharInvHandler {
    type StackTyHandler = StackTyInvHandler;
    type CashHandler = CashInvHandler;
    type EquipHandler = NoopEventHandler<EquipItemSlot>;

    fn create_eq_handler(_svc: Arc<SharedItemSvc>) -> Self::EquipHandler {
        NoopEventHandler::default()
    }

    fn create_stack_handler(svc: Arc<SharedItemSvc>, ty: InventoryType) -> Self::StackTyHandler {
        StackTyInvHandler {
            ops: PendingOperations::default(),
            ty,
            item_svc: svc,
        }
    }

    fn create_cash_handler(svc: Arc<SharedItemSvc>) -> Self::CashHandler {
        CashInvHandler {
            ops: PendingOperations::default(),
            item_svc: svc,
        }
    }
}

#[derive(Debug)]
pub struct CharInventory {
    pub invs: InventorySet<CharInvHandler>,
    pub inv_size: InventorySize,
    recalc_eq_stats: bool,
    eq_ops: PendingOperations,
}

impl CharInventory {
    pub fn from_inv_set(inv_size: InventorySize, inv: InventorySet<NoopInvSetHandler>) -> Self {
        let svc = inv.consume.handler().0.clone();
        Self {
            inv_size,
            invs: inv.with_handler(svc),
            eq_ops: PendingOperations::default(),
            recalc_eq_stats: false,
        }
    }

    pub fn get_updates(&mut self) -> Option<Vec<InventoryOperation>> {
        let mut ops = Vec::new();
        self.eq_ops.drain_into(&mut ops);
        self.invs.consume.handler_mut().ops.drain_into(&mut ops);
        self.invs.misc.handler_mut().ops.drain_into(&mut ops);
        self.invs.etc.handler_mut().ops.drain_into(&mut ops);
        self.invs.cash.handler_mut().ops.drain_into(&mut ops);

        if ops.is_empty() {
            None
        } else {
            Some(ops)
        }
    }

    pub fn get_stack_inv_list(&self, inv_ty: InventoryType) -> ShroomIndexListZ8<Item> {
        self.invs
            .get_stack_inventory(inv_ty)
            .unwrap()
            .item_slots()
            .map(|(slot, item)| (slot as u8 + 1, Item::Stack(item.as_ref().into())))
            .collect()
    }

    pub fn get_cash_inv_list(&self) -> ShroomIndexListZ8<Item> {
        self.invs
            .get_cash_inventory()
            .item_slots()
            .map(|(slot, item)| {
                (
                    slot as u8 + 1,
                    match item {
                        CashItemSlot::Stack(item) => Item::Stack(item.as_ref().into()),
                        CashItemSlot::Pet(item) => Item::Pet(item.as_ref().into()),
                    },
                )
            })
            .collect()
    }

    pub fn inv_size(&self) -> InventorySize {
        self.inv_size
    }

    pub fn get_quantity(&self, id: ItemId) -> anyhow::Result<usize> {
        let ty = id.get_inv_type()?;
        Ok(if ty.is_stack() {
            self.invs.get_stack_inventory(ty).unwrap().quantity_by_id(id)
        } else {
            self.invs
                .get_equipped_inventory(ty)
                .unwrap()
                .item_slots_by_id(&id)
                .count()
        })
    }

    pub fn try_take_all(&mut self, id: ItemId) -> anyhow::Result<usize> {
        let ty = id.get_inv_type()?;
        if ty.is_stack() {
            Ok(self.invs.get_stack_inventory_mut(ty).unwrap().try_take_all_by_id(id)?)
        } else {
            todo!()
        }
    }

    pub fn try_take_by_id(&mut self, id: ItemId, count: usize) -> anyhow::Result<()> {
        let ty = id.get_inv_type()?;
        if ty.is_stack() {
            self.invs.get_stack_inventory_mut(ty).unwrap().try_take_by_id(id, count)?;
        } else {
            todo!()
        }
        Ok(())
    }

    pub fn get_equipped_slot_ids(&self) -> impl Iterator<Item = (CharEquipSlot, ItemId)> + '_ {
        self.invs
            .equipped
            .item_slots()
            .map(|(slot, item)| (slot.into(), item.id()))
    }

    pub fn find_first_throwing_stars(&self, minq_q: usize) -> Option<(usize, &StackItem)> {
        self.invs
            .consume
            .item_slots()
            .find(|(_, item)| item.item_id.is_throwing_star() && item.quantity() >= minq_q)
            .map(|(slot, item)| (slot, item.as_ref()))
    }

    pub fn contains_id(&self, id: &ItemId) -> anyhow::Result<bool> {
        let ty = id.get_inv_type()?;
        Ok(if ty.is_stack() {
            self.invs.get_stack_inventory(ty).unwrap().contains_id(id)
        } else {
            self.invs
                .get_equipped_inventory(ty)
                .unwrap()
                .contains_id(id)
        })
    }

    pub fn add_equip_by_id(&mut self, id: ItemId, data: &ItemService) -> anyhow::Result<usize> {
        self.try_add_equip(data.create_equip(id)?)
    }

    pub fn get_equipped_stats(&self) -> EquipBaseStats {
        self.invs
            .equipped
            .items()
            .map(|item| &item.0.item.stats)
            .fold(EquipBaseStats::default(), |acc, stats| acc + stats.clone())
    }

    pub fn slots(&self, ty: InventoryType) -> usize {
        self.invs.slots(ty)
    }

    pub fn try_add_equip(&mut self, item: EquipItem) -> anyhow::Result<usize> {
        let slot = self.invs.equip.try_add(item.into())?;
        let item = self.invs.equip.get(slot).unwrap().item.as_ref().into();
        self.eq_ops
            .add(InventoryType::Equip, Item::Equip(item), slot as u16 + 1);

        Ok(slot)
    }

    pub fn try_add_stack_item(
        &mut self,
        id: ItemId,
        quantity: usize,
        inv_type: InventoryType,
    ) -> anyhow::Result<()> {
        let inv = self.invs.get_stack_inventory_mut(inv_type)?;
        inv.try_add_stack(id, quantity)?;
        Ok(())
    }

    pub fn equip_item(
        &mut self,
        eq_slot: usize,
        char_equip_slot: CharEquipSlot,
    ) -> anyhow::Result<()> {
        let equip = &mut self.invs.equip;
        let equipped = &mut self.invs.equipped;

        // Take the item from the equip
        let eq_item: EquipItemSlot = equip.try_remove(eq_slot)?;

        // Put the item into the equipped slot
        let prev_item = equipped
            .replace(char_equip_slot.into(), eq_item.into())
            .expect("equip");

        // Put unequipped item back into the equip
        if let Some(item) = prev_item {
            equip.set(eq_slot, item.0)?;
        };

        let dst = -(char_equip_slot as i16);
        // Add pending operation
        self.eq_ops
            .mov(InventoryType::Equip, eq_slot as u16 + 1, dst as u16);
        self.recalc_eq_stats = true;

        Ok(())
    }

    pub fn unequip_item(
        &mut self,
        char_equip_slot: CharEquipSlot,
        eq_slot: Option<usize>,
    ) -> anyhow::Result<()> {
        let equip = &mut self.invs.equip;
        let equipped = &mut self.invs.equipped;

        // Either use the destination slot or create a free slot
        let eq_slot = eq_slot
            .or_else(|| equip.find_free_slot())
            .ok_or(InvError::Full)?;

        // Ensure the eq slot is free
        if equip.get(eq_slot).is_some() {
            anyhow::bail!("Slot is not free");
        }

        // Remove the equipped item
        let eq_item = equipped.try_remove(char_equip_slot.into())?;

        // Put the item into the free equip slot
        equip.set(eq_slot, eq_item.0)?;

        let src = -(char_equip_slot as i16);
        self.eq_ops
            .mov(InventoryType::Equip, src as u16, eq_slot as u16 + 1);

        self.recalc_eq_stats = true;

        Ok(())
    }

    pub fn drop_item(
        &mut self,
        slot: InventorySlot,
        quantity: Option<usize>,
    ) -> anyhow::Result<Either<EquipItemSlot, DropStackItem>> {
        Ok(match slot {
            InventorySlot::Slot(InventoryType::Equip, _) | InventorySlot::EquippedSlot(_) => {
                Either::Left(self.drop_equip_item(slot)?)
            }
            InventorySlot::Slot(ty, _) => Either::Right(self.drop_stack_item(ty, slot, quantity)?),
        })
    }

    pub fn drop_stack_item(
        &mut self,
        inv_type: InventoryType,
        slot: InventorySlot,
        quantity: Option<usize>,
    ) -> anyhow::Result<DropStackItem> {
        let inv = self.invs.get_stack_inventory_mut(inv_type)?;
        let q = quantity.unwrap_or(1); // TODO opt slot quantity
        let (id, q) = inv.take_quantity(slot.as_slot(), q)?;
        Ok(DropStackItem(id, q))
    }

    pub fn drop_equip_item(&mut self, slot: InventorySlot) -> anyhow::Result<EquipItemSlot> {
        Ok(match slot {
            InventorySlot::Slot(_, _) => {
                let item = self.invs.equip.try_remove(slot.as_slot())?;
                self.eq_ops
                    .remove(InventoryType::Equip, slot.as_slot_index());
                self.recalc_eq_stats = true;
                item
            }
            InventorySlot::EquippedSlot(eq_slot) => {
                let item = self.invs.equipped.try_remove(eq_slot.into())?;
                self.eq_ops
                    .remove(InventoryType::Equip, slot.as_slot_index());
                self.recalc_eq_stats = true;
                item.0
            }
        })
    }

    pub fn move_item(
        &mut self,
        src: InventorySlot,
        dst: InventorySlot,
        count: Option<usize>,
    ) -> anyhow::Result<()> {
        if src.inv_type() != dst.inv_type() {
            anyhow::bail!("Inventory type mismatch");
        }

        let inv_type = src.inv_type();

        if inv_type.is_stack() {
            let inv = self.invs.get_stack_inventory_mut(inv_type)?;
            inv.r#move(src.as_slot(), dst.as_slot(), count)?;
        } else {
            if inv_type != InventoryType::Equip {
                anyhow::bail!("Not equip");
            }
            match (src, dst) {
                // Unequip
                (InventorySlot::EquippedSlot(equip), InventorySlot::Slot(_, slot)) => {
                    self.unequip_item(equip, Some(slot as usize))?;
                }
                // Special case without pre-selected equip slot
                (
                    InventorySlot::EquippedSlot(equip),
                    InventorySlot::EquippedSlot(CharEquipSlot::Hat), //TODO hat?
                ) => {
                    self.unequip_item(equip, None)?;
                }
                (InventorySlot::Slot(_, slot), InventorySlot::EquippedSlot(equip)) => {
                    self.equip_item(slot as usize, equip)?;
                }
                (InventorySlot::EquippedSlot(src_), InventorySlot::EquippedSlot(dst_)) => {
                    if !src_.can_swap(&dst_) {
                        anyhow::bail!("Unable to swap");
                    }

                    self.invs.equipped.swap(src_.into(), dst_.into())?;
                    self.eq_ops.mov(
                        InventoryType::Equip,
                        src.as_slot_index(),
                        dst.as_slot_index(),
                    );
                }
                (InventorySlot::Slot(_, _), InventorySlot::Slot(_, _)) => {
                    self.invs.equip.swap(src.as_slot(), dst.as_slot())?;
                    self.eq_ops
                        .mov(inv_type, src.as_slot_index(), dst.as_slot_index());
                }
            }
        }

        Ok(())
    }

    pub fn get_pet(&self, slot: usize) -> Option<&PetItem> {
        self.invs
            .get_cash_inventory()
            .get(slot)
            .and_then(|i| i.as_pet())
            .map(|v| v.as_ref())
    }

    fn update_eq(
        &mut self,
        eq_slot: InventorySlot,
        con: InventorySlot,
        update: impl Fn(&mut EquipItem, ItemId) -> UpgradeResult,
    ) -> anyhow::Result<UpgradeResult> {
        let equip = &mut self.invs.equip;
        let equipped = &mut self.invs.equipped;
        let eq = match eq_slot {
            InventorySlot::EquippedSlot(slot) => {
                &mut equipped
                    .get_mut(slot.into())
                    .context("No Equipped item")?
                    .0
                    .item
            }
            InventorySlot::Slot(InventoryType::Equip, slot) => {
                &mut equip.get_mut(slot.into()).context("No equip item")?.item
            }
            _ => anyhow::bail!("Not equipped slot"),
        };

        let upgrade_item = self
            .invs
            .consume
            .get(con.as_slot())
            .context("No upgrade item")?;
        let res = update(eq, upgrade_item.item_id);
        match res {
            UpgradeResult::Success | UpgradeResult::Failed => {
                let item = eq.as_ref().into();
                self.eq_ops.update(
                    InventoryType::Equip,
                    eq_slot.as_slot_index(),
                    Item::Equip(item),
                );
            }
            UpgradeResult::Destroyed => {
                self.eq_ops
                    .remove(InventoryType::Equip, eq_slot.as_slot_index());
            }
            UpgradeResult::InvalidUpgrade | UpgradeResult::NoSlots => return Ok(res),
        }

        // Consume scroll
        self.invs
            .consume
            .take_quantity(con.as_slot(), 1)
            .expect("consume scroll");

        Ok(res)
    }

    pub fn apply_scroll(
        &mut self,
        eq_slot: InventorySlot,
        scroll_slot: InventorySlot,
        protected: bool,
        meta: &'static MetaService,
    ) -> anyhow::Result<UpgradeResult> {
        // If protected is set, find the protect item
        let protect_slot = protected
            .then(|| {
                self.invs
                    .consume
                    .get_slot_by_id(&ItemId::WHITE_SCROLL)
                    .context("No protect item")
            })
            .transpose()?;

        let res = self.update_eq(eq_slot, scroll_slot, |eq, id| {
            let upgrade = meta.items().consume.get(&id).unwrap();
            eq.apply_upgrade(upgrade, protected, &mut thread_rng())
        })?;

        if matches!(
            res,
            UpgradeResult::Success | UpgradeResult::Failed | UpgradeResult::Destroyed
        ) {
            if let Some(protect_slot) = protect_slot {
                self.invs
                    .consume
                    .take_quantity(protect_slot, 1)
                    .expect("consume protect");
            }
        }

        Ok(res)
    }

    pub fn apply_enhance(
        &mut self,
        eq_slot: InventorySlot,
        scroll_slot: InventorySlot,
        meta: &'static MetaService,
    ) -> anyhow::Result<UpgradeResult> {
        self.update_eq(eq_slot, scroll_slot, |eq, id| {
            let upgrade = meta.items().consume.get(&id).unwrap();
            eq.apply_enhancement(upgrade.id, &mut thread_rng())
        })
    }
}
