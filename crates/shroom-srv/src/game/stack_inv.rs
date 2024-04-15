use super::inventory::{InvError, InvEventHandler, InvItem, InvResult, InvSlotIndex, Inventory, ItemSlotsIdIter, ItemSlotsIdIterMut};

// TODO remember last freeslot per id

pub trait StackInvEventHandler: InvEventHandler {
    fn on_quantity_change(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex);
    fn new_stack(&mut self, id: <Self::Item as InvItem>::Id, quantity: usize) -> Self::Item;
}

pub trait InvStackItem: InvItem + Sized {
    fn max_stack_size(&self) -> usize;
    fn quantity(&self) -> usize;
    fn set_quantity(&mut self, count: usize);

    fn split(&mut self, split_quantity: usize) -> InvResult<(Self::Id, usize)> {
        if split_quantity > self.quantity() {
            return Err(InvError::InsufficentItems(split_quantity - self.quantity()));
        }

        self.sub_quantity(split_quantity)?;
        Ok((self.id(), split_quantity))
    }

    fn free_space(&self) -> usize {
        self.max_stack_size() - self.quantity()
    }

    fn sub_quantity(&mut self, delta: usize) -> InvResult<()> {
        self.set_quantity(
            self.quantity()
                .checked_sub(delta)
                .ok_or(InvError::SlotInsufficentSpace)?,
        );

        Ok(())
    }

    fn add_quantity(&mut self, delta: usize) -> InvResult<()> {
        if self.free_space() < delta {
            return Err(InvError::SlotFull);
        }

        self.set_quantity(self.quantity() + delta);

        Ok(())
    }

    fn merge_into(&mut self, other: &mut Self) -> InvResult<usize> {
        if self.id() != other.id() {
            return Err(InvError::InvalidMergeId);
        }
        let free_space = other.free_space();
        let delta = free_space.min(self.quantity());
        other.add_quantity(delta).unwrap();
        self.sub_quantity(delta).unwrap();
        Ok(delta)
    }
}

#[derive(Debug)]
pub struct StackInventory<T: InvStackItem, H> {
    inv: Inventory<T, H>,
}

impl<T: InvStackItem, H: StackInvEventHandler<Item = T>> StackInventory<T, H> {
    pub fn new(handler: H, capacity: usize) -> Self {
        Self {
            inv: Inventory::new(handler, capacity),
        }
    }

    pub fn with_handler<U: StackInvEventHandler<Item = T>>(self, handler: U) -> StackInventory<T, U> {
        StackInventory {
            inv: self.inv.with_handler(handler),
        }
    }

    pub fn from_iter<I: Iterator<Item = (T::SlotIndex, T)>>(iter: I, handler: H, cap: usize) -> Self {
        Self {
            inv: Inventory::from_iter(iter, handler, cap),
        }
    }

    /// Add a new item into a free slot
    pub fn try_add(&mut self, item: T) -> InvResult<T::SlotIndex> {
        self.inv.try_add(item)
    }

    /// Adds a given amount of stack items
    pub fn try_add_stack(&mut self, id: T::Id, quantity: usize) -> InvResult<()> {
        // TODO check unique here
        if quantity == 0 {
            return Ok(());
        }

        let mut remaining = quantity;

        for slot in self.inv.id_slots.indices_iter(&id) {
            let item = &mut self.inv.slots[slot.to_ix()];
            let free_space = item.free_space();
            if free_space > 0 {
                let delta = free_space.min(remaining);
                item.add_quantity(delta).expect("merge add quantity");
                remaining -= delta;
                self.inv.handler.on_quantity_change(item, slot);
            }

            if remaining == 0 {
                return Ok(());
            }
        }

        // Add the rest as a new stack
        if remaining > 0 {
            let stack = self.inv.handler.new_stack(id, quantity);
            self.inv.try_add(stack).expect("try add stack");
        }

        Ok(())
    }

    pub fn r#move(
        &mut self,
        src: T::SlotIndex,
        dst: T::SlotIndex,
        quantity: Option<usize>,
    ) -> InvResult<()> {
        let src_item = self.inv.get(src).ok_or(InvError::EmptySlot(src.to_ix()))?;

        let quantity = quantity.unwrap_or(src_item.quantity());
        if quantity > src_item.quantity() {
            return Err(InvError::InsufficentItems(quantity - src_item.quantity()));
        }
        let complete_move = quantity == src_item.quantity();



        // Handle empty dst slot
        let Some(dst_item) = self.inv.get(dst) else {
            if complete_move {
                self.inv.swap(src, dst)?;
            } else {
                let split = self.inv.get_mut(src).unwrap().split(quantity)?;
                self.inv.handler.on_quantity_change(&self.inv.slots[src.to_ix()], src);
                let split = self.inv.handler.new_stack(split.0, split.1);
                self.inv.set(dst, split).expect("Set new slot for split");
            }

            return Ok(());
        };

        // Ensure src and dst have the same id
        if src_item.id() != dst_item.id() {
            // Do a swap
            self.inv.swap(src, dst)?;
            return Ok(())
        }

        // Do the merge
        self.add_quantity(dst, quantity)?;
        self.take_quantity(src, quantity)?;

        Ok(())
    }

    pub fn add_quantity(&mut self, slot: T::SlotIndex, quantity: usize) -> InvResult<()> {
        let item = self
            .inv
            .get_mut(slot)
            .ok_or(InvError::EmptySlot(slot.to_ix()))?;
        item.add_quantity(quantity)?;
        self.inv
            .handler
            .on_quantity_change(&self.inv.slots[slot.to_ix()], slot);

        Ok(())
    }

    pub fn take_quantity(&mut self, slot: T::SlotIndex, quantity: usize) -> InvResult<(T::Id, usize)> {
        let item = self
            .inv
            .get_mut(slot)
            .ok_or(InvError::EmptySlot(slot.to_ix()))?;
        let id = item.id();

        if item.quantity() == quantity {
            self.inv.try_remove(slot).expect("take remove");
        } else {
            item.sub_quantity(quantity)?;
            self.inv
                .handler
                .on_quantity_change(&self.inv.slots[slot.to_ix()], slot);
        }
        Ok((id, quantity))
    }

    pub fn adjust_quantity(&mut self, slot: T::SlotIndex, quantity: isize) -> InvResult<()> {
        if quantity < 0 {
            self.take_quantity(slot, quantity.unsigned_abs())?;
            Ok(())
        } else {
            self.add_quantity(slot, quantity as usize)
        }
    }


    pub fn quantity_by_id(&self, id: T::Id) -> usize {
        self.inv
            .item_slots_by_id(&id)
            .map(|(_, item)| item.quantity())
            .sum()
    }

    pub fn try_take_all_by_id(&mut self, id: T::Id) -> InvResult<usize> {
        let slot_for_id = self.inv.slots_by_id(&id).collect::<Vec<_>>();
        let mut total_q = 0;
        for slot in slot_for_id {
            let item = self.inv.get(slot).expect("get slot");
            let q = item.quantity();
            total_q += 1;
            self.take_quantity(slot, q).expect("take quantity");
        }
        Ok(total_q)
    }

    pub fn try_take_by_id(&mut self, id: T::Id, quantity: usize) -> InvResult<()> {
        // Check the quantity exists
        if self.quantity_by_id(id) < quantity {
            return Err(InvError::InsufficentItems(quantity));
        }

        let mut remaining = quantity;
        // TODO: Try to avoid collecting here or use
        // a smaller vector
        let slot_for_id = self.inv.slots_by_id(&id).collect::<Vec<_>>();
        let mut slots_iter = slot_for_id.iter().copied();

        while remaining > 0 {
            let slot = slots_iter.next().expect("slot for id");
            let delta = self.inv.get(slot).expect("get slot").quantity().min(remaining);
            self.take_quantity(slot, delta).expect("take quantity");
            remaining -= delta;
        }

        Ok(())
    }

    pub fn set(&mut self, slot: T::SlotIndex, item: T) -> InvResult<()> {
        self.inv.set(slot, item)
    }

    pub fn get(&self, slot: T::SlotIndex) -> Option<&T> {
        self.inv.get(slot)
    }

    pub fn handler(&self) -> &H {
        &self.inv.handler
    }

    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.inv.handler
    }

    pub fn len(&self) -> usize {
        self.inv.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inv.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.inv.capacity()
    }


    pub fn contains_id(&self, id: &T::Id) -> bool {
        self.inv.contains_id(id)
    }

    pub fn items(&self) -> impl Iterator<Item = &T> + '_ {
        self.inv.items()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.inv.items_mut()
    }

    pub fn item_slots(&self) -> impl Iterator<Item = (T::SlotIndex, &T)> + '_ {
        self.inv.item_slots()
    }

    pub fn item_slots_mut(&mut self) -> impl Iterator<Item = (T::SlotIndex, &mut T)> + '_ {
        self.inv.item_slots_mut()
    }

    pub fn items_by_id(&self, id: &T::Id) -> impl Iterator<Item = &T> + '_ {
        self.inv.items_by_id(id)
    }

    pub fn slots_by_id(&self, id: &T::Id) -> impl Iterator<Item = T::SlotIndex> + '_ {
        self.inv.slots_by_id(id)
    }

    pub fn item_slots_by_id(
        &self,
        id: &T::Id,
    ) -> ItemSlotsIdIter<T> {
        self.inv.item_slots_by_id(id)
    }

    pub fn item_slots_by_id_mut(
        &mut self,
        id: &T::Id,
    ) -> ItemSlotsIdIterMut<T> {
        self.inv.item_slots_by_id_mut(id)
    }

    pub fn get_slot_by_id(&self, id: &T::Id) -> Option<T::SlotIndex> {
        self.item_slots_by_id(id).next().map(|(slot, _)| slot)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct DummyItem(u32, usize);

    impl InvItem for DummyItem {
        type Id = u32;
        type SlotIndex = usize;

        fn id(&self) -> Self::Id {
            self.0
        }
    }

    impl InvStackItem for DummyItem {
        fn quantity(&self) -> usize {
            self.1
        }

        fn set_quantity(&mut self, count: usize) {
            self.1 = count;
        }

        fn max_stack_size(&self) -> usize {
            255
        }
    }

    #[derive(Debug, Default)]
    pub struct DummyHandler(String);

    impl DummyHandler {
        pub fn take(&mut self) -> String {
            std::mem::take(&mut self.0)
        }
    }

    impl InvEventHandler for DummyHandler {
        type Item = DummyItem;

        fn on_add(&mut self, item: &Self::Item, slot: usize) {
            self.0.push_str(&format!("a:{};{slot}-", item.quantity()));
        }

        fn on_remove(&mut self, item: &Self::Item, slot: usize) {
            self.0.push_str(&format!("r:{};{slot}-", item.quantity()));
        }

        fn on_update(&mut self, item: &Self::Item, slot: usize) {
            self.0.push_str(&format!("u:{};{slot}-", item.quantity()));
        }

        fn on_swap(&mut self, slot_a: usize, slot_b: usize) {
            self.0.push_str(&format!("s:{slot_a};{slot_b}-"));
        }
    }

    impl StackInvEventHandler for DummyHandler {
        fn on_quantity_change(&mut self, item: &Self::Item, slot: usize) {
            self.0.push_str(&format!("u:{};{slot}-", item.quantity()));
        }
        
        fn new_stack(&mut self, id: <Self::Item as InvItem>::Id, quantity: usize) -> Self::Item {
            DummyItem(id, quantity)
        }
    }

    fn inv(cap: usize) -> StackInventory<DummyItem, DummyHandler> {
        StackInventory::new(DummyHandler::default(), cap)
    }

    #[test]
    fn stack_move_free_slot() {
        let mut inv = inv(10);
        inv.set(0, DummyItem(1, 10)).unwrap();
        inv.handler_mut().take();

        // Move stack at 0 to 1 completely
        inv.r#move(0, 1, Some(10)).expect("Move");

        //assert!(inv.get(0).is_none());
        assert_eq!(inv.get(1).unwrap().quantity(), 10);
        assert_eq!(inv.handler_mut().take(), "s:0;1-");

        // Move Partial back
        inv.r#move(1, 0, Some(3)).expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 3);
        assert_eq!(inv.get(1).unwrap().quantity(), 7);
        assert_eq!(inv.handler_mut().take(), "u:7;1-a:3;0-");
    }

    #[test]
    fn stack_move_complete() {
        let mut inv = inv(10);
        inv.set(0, DummyItem(1, 9)).unwrap();
        inv.set(1, DummyItem(1, 1)).unwrap();

        // Complete
        inv.r#move(0, 1, Some(9)).expect("Move");
        //assert!(inv.get(0).is_none());
        assert_eq!(inv.get(1).unwrap().quantity(), 10);
    }

    #[test]
    fn stack_move_partial() {
        let mut inv = inv(10);
        inv.set(0, DummyItem(1, 5)).unwrap();
        inv.set(1, DummyItem(1, 5)).unwrap();

        // Complete
        inv.r#move(0, 1, Some(4)).expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 1);
        assert_eq!(inv.get(1).unwrap().quantity(), 9);

        inv.r#move(1, 0, Some(8)).expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 9);
        assert_eq!(inv.get(1).unwrap().quantity(), 1);
    }
}
