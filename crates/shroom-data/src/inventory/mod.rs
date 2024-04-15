pub mod index_map;
pub mod stack;

use index_map::IdIndexMap;
use thiserror::Error;

pub trait InvEventHandler {
    type Item: InvItem;

    fn on_add(&mut self, item: &Self::Item, slot: usize);
    fn on_remove(&mut self, item: &Self::Item, slot: usize);
    fn on_update(&mut self, item: &Self::Item, slot: usize);
    fn on_swap(&mut self, slot_a: usize, slot_b: usize);
}

impl<'a, T: InvEventHandler> InvEventHandler for &'a mut T {
    type Item = T::Item;

    fn on_add(&mut self, item: &Self::Item, slot: usize) {
        T::on_add(self, item, slot)
    }

    fn on_remove(&mut self, item: &Self::Item, slot: usize) {
        T::on_remove(self, item, slot)
    }

    fn on_update(&mut self, item: &Self::Item, slot: usize) {
        T::on_update(self, item, slot)
    }

    fn on_swap(&mut self, slot_a: usize, slot_b: usize) {
        T::on_swap(self, slot_a, slot_b)
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

pub trait InvItemId: Eq + std::hash::Hash + Copy + Clone + Default + std::fmt::Debug {
    fn is_unique(&self) -> bool;
}

pub trait InvSlotIndex: Copy + Clone + std::fmt::Debug {
    fn to_slot_index(&self) -> usize;
    fn from_slot_index(slot: usize) -> Self;
}

impl InvSlotIndex for usize {
    fn to_slot_index(&self) -> usize {
        *self
    }

    fn from_slot_index(slot: usize) -> Self {
        slot
    }
}

pub trait InvItem {
    type Id: InvItemId;
    type SlotIndex: InvSlotIndex;
    fn id(&self) -> Self::Id;
}

#[derive(Debug)]
pub struct Inventory<Item: InvItem, const CAP: usize> {
    slots: Box<[Option<Item>]>,
    len: usize,
    ids: IdIndexMap<Item::Id>,
}

impl<Item: InvItem, const CAP: usize> Default for Inventory<Item, CAP> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<Item: InvItem, const CAP: usize> Inventory<Item, CAP> {
    pub fn empty() -> Self {
        Self {
            slots: (0..CAP).map(|_| None).collect::<Vec<_>>().into_boxed_slice(),
            len: 0,
            ids: IdIndexMap::default(),
        }
    }

    pub fn contains_id(&self, id: &Item::Id) -> bool {
        self.ids.contains_id(id)
    }

    pub fn new(_slots: usize) -> Self {
        //TODO
        Self::empty()
    }

    pub fn slots(&self) -> usize {
        // TODO
        CAP
    }

    fn check_slot(&self, slot: usize) -> InvResult<()> {
        if slot >= self.slots.len() {
            return Err(InvError::InvalidSlot(slot));
        }

        Ok(())
    }

    pub fn check_full(&self) -> InvResult<()> {
        if self.len == self.slots.len() {
            return Err(InvError::Full);
        }

        Ok(())
    }

    fn check_unique(&self, id: &Item::Id) -> InvResult<()> {
        if id.is_unique() && self.ids.contains_id(id) {
            return Err(InvError::UniqueConflict);
        }

        Ok(())
    }

    pub fn find_free_slot(&mut self) -> Option<Item::SlotIndex> {
        self.slots
            .iter()
            .position(|slot| slot.is_none())
            .map(Item::SlotIndex::from_slot_index)
    }

    pub fn get_slot_opt(&self, slot: Item::SlotIndex) -> InvResult<Option<&Item>> {
        let slot = slot.to_slot_index();
        self.check_slot(slot)?;

        Ok(self.slots[slot].as_ref())
    }

    pub fn add(&mut self, item: Item) -> InvResult<Item::SlotIndex> {
        self.check_full()?;
        let free_slot = self.find_free_slot().ok_or(InvError::Full)?;
        self.set(free_slot, item)?;
        Ok(free_slot)
    }

    pub fn replace(&mut self, slot: Item::SlotIndex, item: Item) -> InvResult<Option<Item>> {
        let slot = slot.to_slot_index();
        self.check_slot(slot)?;
        self.check_unique(&item.id())?;

        if let Some(slot_item) = self.slots[slot].as_ref() {
            // Elsewise remove the id link
            self.ids.remove(slot_item.id(), slot);
        } else {
            // If the slot was empty we increment the count
            self.len += 1;
        }

        // Insert the slot into the map
        self.ids.insert(item.id(), slot);
        Ok(self.slots[slot].replace(item))
    }

    pub fn set(&mut self, slot: Item::SlotIndex, item: Item) -> InvResult<()> {
        let s = slot;
        let slot = s.to_slot_index();
        self.check_slot(slot)?;
        self.check_unique(&item.id())?;
        if self.slots[slot].is_some() {
            return Err(InvError::SlotFull);
        }

        self.len += 1;
        self.ids.insert(item.id(), slot);
        self.slots[slot] = Some(item);
        Ok(())
    }

    pub fn remove(&mut self, slot: Item::SlotIndex) -> InvResult<Option<Item>> {
        let slot = slot.to_slot_index();
        Ok(if let Some(item) = self.slots[slot].take() {
            self.ids.remove(item.id(), slot);
            self.len -= 1;
            Some(item)
        } else {
            None
        })
    }

    pub fn take(&mut self, slot: Item::SlotIndex) -> InvResult<Item> {
        self.remove(slot)?
            .ok_or(InvError::EmptySlot(slot.to_slot_index()))
    }

    pub fn swap(&mut self, slot_a: Item::SlotIndex, slot_b: Item::SlotIndex) -> InvResult<()> {
        let slot_a = slot_a.to_slot_index();
        let slot_b = slot_b.to_slot_index();
        self.check_slot(slot_a)?;
        self.check_slot(slot_b)?;
        self.slots.swap(slot_a, slot_b);

        // Item was moved from b to a
        if let Some(slot_a_item) = self.slots[slot_a].as_ref() {
            self.ids.update(&slot_a_item.id(), slot_b, slot_a)?;
        }

        // Item was moved from a to b
        if let Some(slot_b_item) = self.slots[slot_b].as_ref() {
            self.ids.update(&slot_b_item.id(), slot_a, slot_b)?;
        }

        Ok(())
    }

    pub fn get_pair(
        &self,
        (a, b): (Item::SlotIndex, Item::SlotIndex),
    ) -> InvResult<(Option<&Item>, Option<&Item>)> {
        let a = a.to_slot_index();
        let b = b.to_slot_index();
        self.check_slot(a)?;
        self.check_slot(b)?;
        if a == b {
            return Err(InvError::InvalidSlot(a));
        }

        Ok((self.slots[a].as_ref(), self.slots[b].as_ref()))
    }

    pub fn get_pair_mut(
        &mut self,
        (a, b): (Item::SlotIndex, Item::SlotIndex),
    ) -> InvResult<(Option<&mut Item>, Option<&mut Item>)> {
        let a = a.to_slot_index();
        let b = b.to_slot_index();
        self.check_slot(a)?;
        self.check_slot(b)?;
        if a == b {
            return Err(InvError::InvalidSlot(a));
        }

        Ok(if b > a {
            let (x, y) = self.slots.split_at_mut(b);
            (x[a].as_mut(), y[0].as_mut())
        } else {
            // a > b
            let (x, y) = self.slots.split_at_mut(a);
            (y[0].as_mut(), x[b].as_mut())
        })
    }

    pub fn get(&self, slot: Item::SlotIndex) -> InvResult<&Item> {
        let slot = slot.to_slot_index();
        self.check_slot(slot)?;
        self.slots[slot].as_ref().ok_or(InvError::EmptySlot(slot))
    }

    pub fn get_mut(&mut self, slot: Item::SlotIndex) -> InvResult<&mut Item> {
        let slot = slot.to_slot_index();
        self.check_slot(slot)?;
        self.slots[slot].as_mut().ok_or(InvError::EmptySlot(slot))
    }

    pub fn item_slots_by_id(&self, id: Item::Id) -> impl Iterator<Item = (usize, &Item)> + '_ {
        self.ids
            .item_slots_iter(id, &self.slots)
            .map(|(i, s)| (i, s.as_ref().unwrap()))
    }

    pub fn items_by_id(&self, id: Item::Id) -> impl Iterator<Item = &Item> + '_ {
        self.ids
            .item_slots_iter(id, &self.slots)
            .map(|(_, s)| s.as_ref().unwrap())
    }

    pub fn item_slots_by_id_mut(
        &mut self,
        id: Item::Id,
    ) -> impl Iterator<Item = (usize, &mut Item)> + '_ {
        unsafe {
            self.ids
                .item_slots_iter_mut(id, &mut self.slots)
                .map(|(i, s)| (i, s.as_mut().unwrap()))
        }
    }

    pub fn items_by_id_mut(&mut self, id: Item::Id) -> impl Iterator<Item = &mut Item> + '_ {
        unsafe {
            self.ids
                .item_slots_iter_mut(id, &mut self.slots)
                .map(|(_, s)| s.as_mut().unwrap())
        }
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.slots.iter().filter_map(|s| s.as_ref())
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.slots.iter_mut().filter_map(|s| s.as_mut())
    }

    pub fn item_with_slots(&self) -> impl Iterator<Item = (usize, &Item)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|s| (i, s)))
    }

    pub fn item_with_slots_mut(&mut self) -> impl Iterator<Item = (usize, &mut Item)> {
        self.slots
            .iter_mut()
            .enumerate()
            .filter_map(|(i, s)| s.as_mut().map(|s| (i, s)))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.slots.len()
    }

    pub fn free_slots(&self) -> usize {
        self.capacity() - self.len
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
}

impl<Item, const CAP: usize> FromIterator<(usize, Item)> for Inventory<Item, CAP>
where
    Item: InvItem + Default,
{
    fn from_iter<T: IntoIterator<Item = (usize, Item)>>(iter: T) -> Self {
        let mut inv = Self::default();
        for (slot, item) in iter {
            inv.set(Item::SlotIndex::from_slot_index(slot), item)
                .unwrap();
        }
        inv
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

    impl InvItemId for u32 {
        fn is_unique(&self) -> bool {
            self % 2 == 0
        }
    }
    impl InvItem for DummyItem {
        type Id = u32;
        type SlotIndex = usize;

        fn id(&self) -> Self::Id {
            self.0
        }
    }

    pub type Inv2<Item> = Inventory<Item, 2>;
    pub type Inv4<Item> = Inventory<Item, 4>;

    #[test]
    fn add_item_unique() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        assert_eq!(inv.len(), 1);

        assert_eq!(inv.add(DummyItem::new(0)), Err(InvError::UniqueConflict));
        assert_eq!(inv.len(), 1);

        // Item should be inserted in the first slot
        assert!(inv.get(0).is_ok())
    }

    #[test]
    fn add_item_full() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();
        assert_eq!(inv.len(), 2);

        assert_eq!(inv.add(DummyItem::new(2)), Err(InvError::Full));
        assert_eq!(inv.len(), 2);
    }

    #[test]
    fn remove_item() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();

        assert_eq!(inv.len(), 2);
        assert_eq!(inv.remove(0).unwrap().unwrap().id(), 0);
        assert_eq!(inv.len(), 1);
        assert_eq!(inv.remove(1).unwrap().unwrap().id(), 1);
        assert_eq!(inv.len(), 0);
    }

    #[test]
    fn pair_mut() {
        let mut inv = Inv2::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();

        let (i1, i2) = inv.get_pair_mut((0, 1)).unwrap();
        assert_eq!(i1.unwrap().id(), 0);
        assert_eq!(i2.unwrap().id(), 1);
    }

    #[test]
    fn swap() {
        let mut inv = Inv4::empty();
        inv.add(DummyItem::new(0)).unwrap();
        inv.add(DummyItem::new(1)).unwrap();

        assert_eq!(inv.items_by_id(0).count(), 1);
        assert_eq!(inv.items_by_id(1).count(), 1);

        // Empty slot swap works
        inv.swap(2, 3).unwrap();
        // Invalid slot is an error
        assert!(inv.swap(3, 4).is_err());

        // Move the item to an empty slot
        inv.swap(0, 2).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(2).unwrap().id(), 0);
        // Slot 0 is empty
        assert_eq!(inv.get(0).unwrap_err(), InvError::EmptySlot(0));
        // Still one item with id 0
        assert_eq!(inv.items_by_id(0).count(), 1);
        // But It's at index 2
        assert_eq!(inv.item_slots_by_id(0).next().unwrap().0, 2);

        // Move empty slot to an occupied slot
        inv.swap(0, 2).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(0).unwrap().id(), 0);
        // Slot 0 is empty
        assert_eq!(inv.get(2).unwrap_err(), InvError::EmptySlot(2));
        // Still one item with id 0
        assert_eq!(inv.items_by_id(0).count(), 1);
        // But It's at index 2
        assert_eq!(inv.item_slots_by_id(0).next().unwrap().0, 0);

        // Swap first two occupied slots
        inv.swap(0, 1).unwrap();
        // Item is now at slot 2
        assert_eq!(inv.get(0).unwrap().id(), 1);
        assert_eq!(inv.get(1).unwrap().id(), 0);

        assert_eq!(inv.items_by_id(0).count(), 1);
        assert_eq!(inv.items_by_id(1).count(), 1);

        assert_eq!(inv.item_slots_by_id(0).next().unwrap().0, 1);
        assert_eq!(inv.item_slots_by_id(1).next().unwrap().0, 0);
    }

    #[test]
    fn items_by_id() {
        let mut inv = Inv2::empty();
        assert_eq!(inv.items_by_id(0).count(), 0);
        inv.add(DummyItem::new(0)).unwrap();
        assert_eq!(inv.items_by_id(0).count(), 1);
        inv.add(DummyItem::new(1)).unwrap();
        assert_eq!(inv.items_by_id(1).count(), 1);

        assert_eq!(inv.item_slots_by_id_mut(0).count(), 1);
        assert_eq!(inv.item_slots_by_id_mut(1).count(), 1);

        assert_eq!(inv.remove(0).unwrap().unwrap().id(), (0));
        assert_eq!(inv.items_by_id(0).count(), 0);
        assert_eq!(inv.items_by_id(1).count(), 1);
        assert_eq!(inv.remove(1).unwrap().unwrap().id(), (1));
        assert_eq!(inv.items_by_id(1).count(), 0);
    }
}
