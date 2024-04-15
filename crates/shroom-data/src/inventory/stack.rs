use super::{InvError, InvEventHandler, InvItem, InvResult, InvSlotIndex, Inventory};

pub trait StackInvEventHandler: InvEventHandler {
    fn on_quantity_change(&mut self, item: &Self::Item, slot: usize);
}

impl<'a, T: StackInvEventHandler> StackInvEventHandler for &'a mut T {
    fn on_quantity_change(&mut self, item: &Self::Item, slot: usize) {
        T::on_quantity_change(self, item, slot)
    }
}

pub trait InvStackItem: InvItem + Sized {
    fn max_stack_size(&self) -> usize;
    fn quantity(&self) -> usize;
    fn set_quantity(&mut self, count: usize);
    fn new_stack(id: Self::Id, quantity: usize) -> Self;

    fn split(&mut self, split_quantity: usize) -> InvResult<Self> {
        if split_quantity > self.quantity() {
            return Err(InvError::InsufficentItems(split_quantity - self.quantity()));
        }

        self.sub_quantity(split_quantity)?;
        Ok(Self::new_stack(self.id(), split_quantity))
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
pub struct StackInv<Item: InvStackItem, const CAP: usize>(Inventory<Item, CAP>);

impl<Item: InvStackItem, const CAP: usize> StackInv<Item, CAP> {
    pub fn new(slots: usize) -> Self {
        Self(Inventory::new(slots))
    }

    pub fn contains_id(&self, id: &Item::Id) -> bool {
        self.0.contains_id(id)
    }

    pub fn slots(&self) -> usize {
        self.0.slots()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn clear_empty_slots(&mut self) {
        for slot in 0..self.0.len() {
            let slot = Item::SlotIndex::from_slot_index(slot);
            if let Ok(Some(item)) = self.0.get_slot_opt(slot) {
                if item.quantity() == 0 {
                    self.0.remove(slot).unwrap();
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn empty() -> Self {
        Self(Inventory::empty())
    }

    pub fn item_quantity_by_id(&self, id: Item::Id) -> usize {
        self.0.items_by_id(id).map(|i| i.quantity()).sum()
    }

    pub fn set(&mut self, slot: Item::SlotIndex, stack: Item) -> InvResult<()> {
        self.0.set(slot, stack)
    }

    pub fn get(&self, slot: Item::SlotIndex) -> InvResult<&Item> {
        self.0.get(slot)
    }

    pub fn get_mut(&mut self, slot: Item::SlotIndex) -> InvResult<&mut Item> {
        self.0.get_mut(slot)
    }

    pub fn add2(&mut self, mut stack: Item) -> InvResult<()> {
        // If the inventory is full we can't
        // add a new item after distribution
        if self.0.is_full() {
            return Err(InvError::Full);
        }

        // Filter empty item stacks
        if stack.quantity() == 0 {
            return Ok(());
        }

        // Attempt to distribute the stack into existing stacks
        for (_, item) in self.0.item_slots_by_id_mut(stack.id()) {
            let free_space = item.free_space();
            if free_space > 0 {
                stack.merge_into(item).expect("Merge failed");

                // If we merged all items into the inventory
                // we are done
                if stack.quantity() == 0 {
                    return Ok(());
                }
            }
        }

        // ... else-wise we need to add the rest
        self.0.add(stack)?;
        Ok(())
    }

    pub fn add(
        &mut self,
        mut stack: Item,
        mut handler: impl StackInvEventHandler<Item = Item>,
    ) -> InvResult<()> {
        // If the inventory is full we can't
        // add a new item after distribution
        if self.0.is_full() {
            return Err(InvError::Full);
        }

        // Filter empty item stacks
        if stack.quantity() == 0 {
            return Ok(());
        }

        // Attempt to distribute the stack into existing stacks
        for (slot, item) in self.0.item_slots_by_id_mut(stack.id()) {
            let free_space = item.free_space();
            if free_space > 0 {
                stack.merge_into(item).expect("Merge failed");
                handler.on_quantity_change(item, slot);

                // If we merged all items into the inventory
                // we are done
                if stack.quantity() == 0 {
                    return Ok(());
                }
            }
        }

        // ... else-wise we need to add the rest
        let slot = self.0.add(stack)?;
        handler.on_add(self.0.get(slot).unwrap(), slot.to_slot_index());
        Ok(())
    }

    /// Attempts to take `n` items from the given slot
    pub fn take_from_slot(
        &mut self,
        slot: Item::SlotIndex,
        n: Option<usize>,
        mut handler: impl StackInvEventHandler<Item = Item>,
    ) -> InvResult<Item> {
        let item = self.0.get_mut(slot)?;

        let split = if let Some(q) = n {
            item.split(q)?
        } else {
            item.split(item.quantity())?
        };
        Ok(if item.quantity() == 0 {
            // Remove the item
            let item = self.0.take(slot).unwrap();
            handler.on_remove(&item, slot.to_slot_index());
            item
        } else {
            handler.on_quantity_change(item, slot.to_slot_index());
            split
        })
    }

    pub fn move_stack(
        &mut self,
        stack_src: Item::SlotIndex,
        stack_dst: Item::SlotIndex,
        quantity: Option<usize>,
        mut handler: impl StackInvEventHandler<Item = Item>,
    ) -> InvResult<()> {
        let (src, dst) = self.0.get_pair_mut((stack_src, stack_dst))?;
        let Some(src) = src else {
            return Err(InvError::EmptySlot(stack_src.to_slot_index()));
        };
        let quantity = quantity.unwrap_or(src.quantity());

        if quantity > src.quantity() {
            return Err(InvError::InsufficentItems(quantity - src.quantity()));
        }

        let complete_move = quantity == src.quantity();

        // Handle empty dst
        let Some(dst) = dst else {
            // Dst is free so check if we remove or keep src
            if complete_move {
                self.0.swap(stack_src, stack_dst)?;
                handler.on_swap(stack_src.to_slot_index(), stack_dst.to_slot_index());
            } else {
                let split = src.split(quantity)?;
                handler.on_quantity_change(src, stack_src.to_slot_index());
                self.0.set(stack_dst, split).expect("Set src slot");
                handler.on_add(self.0.get(stack_dst)?, stack_dst.to_slot_index());
            }

            return Ok(());
        };

        // Ensure it's the same id
        if src.id() != dst.id() {
            return Err(InvError::InvalidMergeId);
        }
        dst.add_quantity(quantity)?;
        handler.on_quantity_change(dst, stack_dst.to_slot_index());

        // Either remove the slot or sub the quantity
        if complete_move {
            let src = self.0.take(stack_src)?;
            handler.on_remove(&src, stack_src.to_slot_index());
        } else {
            src.sub_quantity(quantity).expect("src quantity");
            handler.on_quantity_change(src, stack_src.to_slot_index());
        }
        Ok(())
    }

    /// Attempts to take `n` items from the inventory
    /// of the given item id
    pub fn take_items(&mut self, id: Item::Id, mut n: usize) -> InvResult<()> {
        let q = self.item_quantity_by_id(id);
        // Check if we have items
        if q < n {
            return Err(InvError::InsufficentItems(n - q));
        }

        for item in self.0.items_by_id_mut(id) {
            let delta = item.quantity().min(n);
            n -= delta;
            item.sub_quantity(delta).unwrap();

            if n == 0 {
                break;
            }
        }

        self.clear_empty_slots();

        // Now we take n items out of the inventory

        todo!()
    }

    pub fn take(&mut self, slot: Item::SlotIndex) -> InvResult<Item> {
        self.0.take(slot)
    }

    pub fn item_with_slots(&self) -> impl Iterator<Item = (usize, &Item)> {
        self.0.item_with_slots()
    }

    pub fn item_with_slots_mut(&mut self) -> impl Iterator<Item = (usize, &mut Item)> {
        self.0.item_with_slots_mut()
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.0.items()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.0.items_mut()
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

        fn new_stack(id: Self::Id, quantity: usize) -> Self {
            Self(id, quantity)
        }
    }

    #[derive(Debug, Default)]
    pub struct DummyHandler(String);

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
    }

    #[test]
    fn stack_move_free_slot() {
        let mut inv = StackInv::<DummyItem, 10>::empty();
        inv.set(0, DummyItem(1, 10)).unwrap();

        // Complete
        let mut handler = DummyHandler::default();
        inv.move_stack(0, 1, Some(10), &mut handler).expect("Move");
        //assert!(inv.get(0).is_none());
        assert_eq!(inv.get(1).unwrap().quantity(), 10);
        assert_eq!(handler.0, "s:0;1-");

        // Move Partial back
        let mut handler = DummyHandler::default();
        inv.move_stack(1, 0, Some(3), &mut handler).expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 3);
        assert_eq!(inv.get(1).unwrap().quantity(), 7);
        assert_eq!(handler.0, "u:7;1-a:3;0-");
    }

    #[test]
    fn stack_move_complete() {
        let mut inv = StackInv::<DummyItem, 10>::empty();
        inv.set(0, DummyItem(1, 9)).unwrap();
        inv.set(1, DummyItem(1, 1)).unwrap();

        // Complete
        let mut handler = DummyHandler::default();
        inv.move_stack(0, 1, Some(9), &mut handler).expect("Move");
        //assert!(inv.get(0).is_none());
        assert_eq!(inv.get(1).unwrap().quantity(), 10);
    }

    #[test]
    fn stack_move_partial() {
        let mut inv = StackInv::<DummyItem, 10>::empty();
        inv.set(0, DummyItem(1, 5)).unwrap();
        inv.set(1, DummyItem(1, 5)).unwrap();

        // Complete
        let mut handler = DummyHandler::default();
        inv.move_stack(0, 1, Some(4), &mut handler).expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 1);
        assert_eq!(inv.get(1).unwrap().quantity(), 9);

        let mut handler = DummyHandler::default();
        inv.move_stack(1, 0, Some(8), &mut handler).expect("Move");
        assert_eq!(inv.get(0).unwrap().quantity(), 9);
        assert_eq!(inv.get(1).unwrap().quantity(), 1);
    }
}
