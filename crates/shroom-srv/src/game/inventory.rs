use stable_vec::ExternStableVec;
use thiserror::Error;

use super::id_ix_map::IdIndexMap;

pub trait InvItemId: Eq + std::hash::Hash + Copy + std::fmt::Debug {}

impl<T: Eq + std::hash::Hash + Copy + std::fmt::Debug> InvItemId for T {}

pub trait InvSlotIndex: Copy + Ord + std::fmt::Debug {
    fn to_ix(&self) -> usize;
    fn from_ix(slot: usize) -> Self;
}

impl InvSlotIndex for usize {
    fn to_ix(&self) -> usize {
        *self
    }

    fn from_ix(slot: usize) -> Self {
        slot
    }
}

pub trait InvItem {
    type Id: InvItemId;
    type SlotIndex: InvSlotIndex;
    fn id(&self) -> Self::Id;
}

pub trait InvEventHandler {
    type Item: InvItem;

    fn on_add(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex);
    fn on_remove(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex);
    fn on_update(&mut self, item: &Self::Item, slot: <Self::Item as InvItem>::SlotIndex);
    fn on_swap(
        &mut self,
        slot_a: <Self::Item as InvItem>::SlotIndex,
        slot_b: <Self::Item as InvItem>::SlotIndex,
    );

    fn is_unique(&self, _id: <Self::Item as InvItem>::Id) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct NoopEventHandler<T>(std::marker::PhantomData<T>);

impl<T> Default for NoopEventHandler<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: InvItem> InvEventHandler for NoopEventHandler<T> {
    type Item = T;

    fn on_add(&mut self, _: &Self::Item, _: <Self::Item as InvItem>::SlotIndex) {}
    fn on_remove(&mut self, _: &Self::Item, _: <Self::Item as InvItem>::SlotIndex) {}
    fn on_update(&mut self, _: &Self::Item, _: <Self::Item as InvItem>::SlotIndex) {}
    fn on_swap(
        &mut self,
        _: <Self::Item as InvItem>::SlotIndex,
        _: <Self::Item as InvItem>::SlotIndex,
    ) {
    }
}

pub type InvResult<T> = Result<T, InvError>;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum InvError {
    #[error("Unique item conflict")]
    UniqueConflict,
    #[error("Inventory is full")]
    Full,
    #[error("Slot is full")]
    SlotFull,
    #[error("Slot has insufficent space")]
    SlotInsufficentSpace,
    #[error("Invalid slot {0}")]
    InvalidSlot(usize),
    #[error("Slot is empty")]
    EmptySlot(usize),
    #[error("Insufficent items in slot {0}")]
    InsufficentItems(usize),
    #[error("Invalid merge id")]
    InvalidMergeId,
}

#[derive(Debug)]
pub struct Inventory<T: InvItem, H> {
    pub(crate) slots: ExternStableVec<T>,
    pub(crate) handler: H,
    pub(crate) id_slots: IdIndexMap<T::Id, T::SlotIndex>,
}

pub struct ItemSlotsIdIter<'a, T: InvItem> {
    slots_ix: &'a [T::SlotIndex],
    slots: &'a ExternStableVec<T>,
}

impl<'a, T: InvItem> Iterator for ItemSlotsIdIter<'a, T> {
    type Item = (T::SlotIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&slot) = self.slots_ix.first() {
            self.slots_ix = &self.slots_ix[1..];
            let item = self.slots.get(slot.to_ix()).unwrap();
            Some((slot, item))
        } else {
            None
        }
    }
}

pub struct ItemSlotsIdIterMut<'a, T: InvItem> {
    slots_ix: &'a [T::SlotIndex],
    slots: &'a mut ExternStableVec<T>,
}

impl<'a, T: InvItem> Iterator for ItemSlotsIdIterMut<'a, T> {
    type Item = (T::SlotIndex, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&slot) = self.slots_ix.first() {
            self.slots_ix = &self.slots_ix[1..];
            // Safety: the iterator lives shorter than the mutable reference
            let item = unsafe { std::mem::transmute(self.slots.get_mut(slot.to_ix()).unwrap()) };
            Some((slot, item))
        } else {
            None
        }
    }
}

pub struct InvItemEntry<'a, H: InvEventHandler> {
    inv: &'a mut Inventory<H::Item, H>,
    slot: <H::Item as InvItem>::SlotIndex,
}

impl<'a, H: InvEventHandler> InvItemEntry<'a, H> {
    pub fn update(&mut self) {
        self.inv
            .handler
            .on_update(&self.inv.slots[self.slot.to_ix()], self.slot);
    }

    pub fn remove(self) -> H::Item {
        self.inv.try_remove(self.slot).unwrap()
    }
}

impl<T: InvItem, H: InvEventHandler<Item = T>> Inventory<T, H> {
    pub fn new(handler: H, cap: usize) -> Self {
        Self {
            slots: ExternStableVec::with_capacity(cap),
            id_slots: IdIndexMap::default(),
            handler,
        }
    }

    pub fn with_handler<U: InvEventHandler<Item = T>>(self, handler: U) -> Inventory<T, U> {
        Inventory {
            slots: self.slots,
            id_slots: self.id_slots,
            handler,
        }
    }

    pub fn entry(&mut self, slot: T::SlotIndex) -> InvItemEntry<H> {
        InvItemEntry { inv: self, slot }
    }

    pub fn from_iter<I: Iterator<Item = (T::SlotIndex, T)>>(
        iter: I,
        handler: H,
        cap: usize,
    ) -> Self {
        let mut slots = ExternStableVec::with_capacity(cap);
        let mut id_slots = IdIndexMap::default();
        for (ix, item) in iter {
            id_slots.insert(item.id(), ix);
            slots.insert(ix.to_ix(), item);
        }

        Self {
            slots,
            id_slots,
            handler,
        }
    }

    fn check_slot(&self, slot: usize) -> InvResult<()> {
        if slot >= self.slots.capacity() {
            return Err(InvError::InvalidSlot(slot));
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.slots.num_elements()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    pub fn contains_id(&self, id: &T::Id) -> bool {
        self.id_slots.contains_id(id)
    }

    pub fn find_free_slot(&self) -> Option<T::SlotIndex> {
        self.slots
            .first_empty_slot_from(0)
            .map(T::SlotIndex::from_ix)
    }

    pub fn check_full(&self) -> InvResult<()> {
        if self.slots.is_compact() {
            return Err(InvError::Full);
        }

        Ok(())
    }

    pub fn try_add_with(&mut self, item: impl FnOnce() -> T) -> InvResult<T::SlotIndex> {
        if let Some(slot) = self.slots.first_empty_slot_from(0) {
            let item = item();
            let id = item.id();
            // TODO check before somehow
            if self.handler.is_unique(id) && self.contains_id(&id) {
                return Err(InvError::UniqueConflict);
            }
            self.slots.insert(slot, item);
            self.id_slots.insert(id, T::SlotIndex::from_ix(slot));
            self.handler
                .on_add(&self.slots[slot], T::SlotIndex::from_ix(slot));
            Ok(<T::SlotIndex as InvSlotIndex>::from_ix(slot))
        } else {
            Err(InvError::Full)
        }
    }

    pub fn try_add(&mut self, item: T) -> InvResult<T::SlotIndex> {
        self.try_add_with(move || item)
    }

    pub fn try_remove(&mut self, slot: T::SlotIndex) -> InvResult<T> {
        let slot_ix = slot.to_ix();
        if let Some(item) = self.slots.remove(slot_ix) {
            let item_id = item.id();
            self.id_slots.remove(item_id, slot);
            self.handler.on_remove(&item, slot);
            Ok(item)
        } else {
            Err(InvError::EmptySlot(slot_ix))
        }
    }

    pub fn replace(&mut self, slot: T::SlotIndex, item: T) -> InvResult<Option<T>> {
        let old = self.try_remove(slot).ok();
        self.slots.insert(slot.to_ix(), item);
        Ok(old)
    }

    pub fn set(&mut self, slot: T::SlotIndex, item: T) -> InvResult<()> {
        let ix = slot.to_ix();
        if self.slots.get(ix).is_some() {
            return Err(InvError::SlotFull);
        }
        self.id_slots.insert(item.id(), slot);
        self.slots.insert(ix, item);
        self.handler.on_add(&self.slots[ix], slot);
        Ok(())
    }

    pub fn swap(&mut self, slot_a: T::SlotIndex, slot_b: T::SlotIndex) -> InvResult<()> {
        let slot_a_ix = slot_a.to_ix();
        self.check_slot(slot_a_ix)?;
        let slot_b_ix = slot_b.to_ix();
        self.check_slot(slot_b_ix)?;
        self.slots.swap(slot_a_ix, slot_b_ix);
        if let Some(item) = self.slots.get(slot_a_ix) {
            self.id_slots.update(&item.id(), slot_b, slot_a)?;
        }
        if let Some(item) = self.slots.get(slot_b_ix) {
            self.id_slots.update(&item.id(), slot_a, slot_b)?;
        }
        self.handler.on_swap(slot_a, slot_b);
        Ok(())
    }

    pub fn get(&self, slot: T::SlotIndex) -> Option<&T> {
        self.slots.get(slot.to_ix())
    }

    pub fn get_mut(&mut self, slot: T::SlotIndex) -> Option<&mut T> {
        self.slots.get_mut(slot.to_ix())
    }

    pub fn get_pair(&self, (a, b): (T::SlotIndex, T::SlotIndex)) -> (Option<&T>, Option<&T>) {
        (self.get(a), self.get(b))
    }

    pub fn items(&self) -> impl Iterator<Item = &T> + '_ {
        self.slots.iter().map(|(_, item)| item)
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.slots.iter_mut().map(|(_, item)| item)
    }

    pub fn item_slots(&self) -> impl Iterator<Item = (T::SlotIndex, &T)> + '_ {
        self.slots
            .iter()
            .map(|(ix, item)| (<T::SlotIndex as InvSlotIndex>::from_ix(ix), item))
    }

    pub fn item_slots_mut(&mut self) -> impl Iterator<Item = (T::SlotIndex, &mut T)> + '_ {
        self.slots
            .iter_mut()
            .map(|(ix, item)| (<T::SlotIndex as InvSlotIndex>::from_ix(ix), item))
    }

    pub fn items_by_id(&self, id: &T::Id) -> impl Iterator<Item = &T> + '_ {
        self.id_slots
            .indices_iter(id)
            .map(|slot| &self.slots[slot.to_ix()])
    }

    pub fn slots_by_id(&self, id: &T::Id) -> impl Iterator<Item = T::SlotIndex> + '_ {
        self.id_slots.indices_iter(id)
    }

    pub fn item_slots_by_id(&self, id: &T::Id) -> ItemSlotsIdIter<T> {
        let slots_ix = self.id_slots.indices(id).unwrap_or(&[]);
        ItemSlotsIdIter {
            slots_ix,
            slots: &self.slots,
        }
    }

    pub fn item_slots_by_id_mut(&mut self, id: &T::Id) -> ItemSlotsIdIterMut<T> {
        let slots_ix = self.id_slots.indices(id).unwrap_or(&[]);
        ItemSlotsIdIterMut {
            slots_ix,
            slots: &mut self.slots,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    pub struct DummyItem(u32);

    impl DummyItem {
        pub fn new(id: u32) -> Self {
            Self(id)
        }
    }

    impl InvItem for DummyItem {
        type Id = u32;
        type SlotIndex = usize;

        fn id(&self) -> Self::Id {
            self.0
        }
    }

    pub struct NoopHandler;
    impl InvEventHandler for NoopHandler {
        type Item = DummyItem;

        fn on_add(&mut self, _: &Self::Item, _: <Self::Item as InvItem>::SlotIndex) {}
        fn on_remove(&mut self, _: &Self::Item, _: <Self::Item as InvItem>::SlotIndex) {}
        fn on_update(&mut self, _: &Self::Item, _: <Self::Item as InvItem>::SlotIndex) {}
        fn on_swap(
            &mut self,
            _: <Self::Item as InvItem>::SlotIndex,
            _: <Self::Item as InvItem>::SlotIndex,
        ) {
        }

        fn is_unique(&self, id: <Self::Item as InvItem>::Id) -> bool {
            id % 2 == 0
        }
    }

    fn inv(cap: usize) -> Inventory<DummyItem, NoopHandler> {
        Inventory::new(NoopHandler, cap)
    }

    #[test]
    fn add_item_unique() {
        let mut inv = inv(2);
        inv.try_add(DummyItem::new(0)).unwrap();
        assert_eq!(inv.len(), 1);

        assert_eq!(
            inv.try_add(DummyItem::new(0)),
            Err(InvError::UniqueConflict)
        );
        assert_eq!(inv.len(), 1);

        // Item should be inserted in the first slot
        assert!(inv.get(0).is_some())
    }

    #[test]
    fn add_item_full() {
        let mut inv = inv(2);
        inv.try_add(DummyItem::new(0)).unwrap();
        inv.try_add(DummyItem::new(1)).unwrap();
        assert_eq!(inv.len(), 2);

        assert_eq!(inv.try_add(DummyItem::new(2)), Err(InvError::Full));
        assert_eq!(inv.len(), 2);
    }

    #[test]
    fn remove_item() {
        let mut inv = inv(2);
        inv.try_add(DummyItem::new(0)).unwrap();
        inv.try_add(DummyItem::new(1)).unwrap();

        assert_eq!(inv.len(), 2);
        assert_eq!(inv.try_remove(0).unwrap().id(), 0);
        assert_eq!(inv.len(), 1);
        assert_eq!(inv.try_remove(1).unwrap().id(), 1);
        assert_eq!(inv.len(), 0);
    }

    #[test]
    fn swap() {
        let mut inv = inv(4);
        inv.try_add(DummyItem::new(0)).unwrap();
        inv.try_add(DummyItem::new(1)).unwrap();

        assert_eq!(inv.items_by_id(&0).count(), 1);
        assert_eq!(inv.items_by_id(&1).count(), 1);

        // Empty slot swap works
        inv.swap(2, 3).unwrap();
        // Invalid slot is an error
        assert!(inv.swap(3, 4).is_err());

        // Move the item to an empty slot
        inv.swap(0, 2).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(2).unwrap().id(), 0);
        // Slot 0 is empty
        assert!(inv.get(0).is_none());
        // Still one item with id 0
        assert_eq!(inv.items_by_id(&0).count(), 1);
        // But It's at index 2
        assert_eq!(inv.slots_by_id(&0).next().unwrap(), 2);

        // Move empty slot to an occupied slot
        inv.swap(0, 2).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(0).unwrap().id(), 0);
        // Slot 0 is empty
        assert!(inv.get(2).is_none());
        // Still one item with id 0
        assert_eq!(inv.items_by_id(&0).count(), 1);
        // But It's at index 2
        assert_eq!(inv.slots_by_id(&0).next().unwrap(), 0);

        // Swap first two occupied slots
        inv.swap(0, 1).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(0).unwrap().id(), 1);
        assert_eq!(inv.get(1).unwrap().id(), 0);

        assert_eq!(inv.items_by_id(&0).count(), 1);
        assert_eq!(inv.items_by_id(&1).count(), 1);

        assert_eq!(inv.slots_by_id(&0).next().unwrap(), 1);
        assert_eq!(inv.slots_by_id(&1).next().unwrap(), 0);
    }

    #[test]
    fn items_by_id() {
        let mut inv = inv(2);
        assert_eq!(inv.items_by_id(&0).count(), 0);
        inv.try_add(DummyItem::new(0)).unwrap();
        assert_eq!(inv.items_by_id(&0).count(), 1);
        inv.try_add(DummyItem::new(1)).unwrap();
        assert_eq!(inv.items_by_id(&1).count(), 1);

        assert_eq!(inv.item_slots_by_id_mut(&0).count(), 1);
        assert_eq!(inv.item_slots_by_id_mut(&1).count(), 1);

        assert_eq!(inv.try_remove(0).unwrap().id(), (0));
        assert_eq!(inv.items_by_id(&0).count(), 0);
        assert_eq!(inv.items_by_id(&1).count(), 1);
        assert_eq!(inv.try_remove(1).unwrap().id(), (1));
        assert_eq!(inv.items_by_id(&1).count(), 0);
    }
}
