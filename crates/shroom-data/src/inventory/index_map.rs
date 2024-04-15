use std::collections::{hash_map::Entry, HashMap};

use smallvec::SmallVec;

use super::{InvError, InvResult};

const INITIAL_INDEX_CAPACITY: usize = 8;
type IndexList = SmallVec<[u16; INITIAL_INDEX_CAPACITY]>;

#[derive(Default, Debug)]
pub struct IdIndexMap<Id>(HashMap<Id, IndexList>);

impl<Id: Eq + std::hash::Hash + Copy + Clone> IdIndexMap<Id> {
    /// Insert a slot index for the given id
    pub fn insert(&mut self, id: Id, slot: usize) {
        let slot = slot as u16;
        let ix = self.0.entry(id).or_default();

        // Find insert position
        let pos = ix.iter().position(|&s| s > slot).unwrap_or(ix.len());
        ix.insert(pos, slot);
    }

    pub fn remove(&mut self, id: Id, slot: usize) {
        let slot = slot as u16;
        if let Entry::Occupied(mut entry) = self.0.entry(id) {
            let pos = entry.get().iter().position(|&s| s == slot).unwrap();
            entry.get_mut().remove(pos);
            if entry.get().is_empty() {
                entry.remove();
            }
        }
    }

    pub fn update(&mut self, id: &Id, old_slot: usize, new_slot: usize) -> InvResult<()> {
        let old_slot = old_slot as u16;
        let new_slot = new_slot as u16;
        //TODO use an ID related error
        let ix = self
            .0
            .get_mut(id)
            .ok_or(InvError::EmptySlot(old_slot as usize))?;

        let pos = ix
            .iter()
            .position(|&slot| slot == old_slot)
            .ok_or(InvError::EmptySlot(old_slot as usize))?;
        ix[pos] = new_slot;
        Ok(())
    }

    pub fn contains_id(&self, id: &Id) -> bool {
        self.0.contains_key(id)
    }

    pub fn indices(&self, id: &Id) -> Option<&[u16]> {
        self.0.get(id).map(|s| s.as_slice())
    }

    pub fn indices_iter(&self, id: Id) -> impl Iterator<Item = usize> + '_ {
        self.indices(&id)
            .into_iter()
            .flatten()
            .map(|i| *i as usize)
    }

    // TODO(!!!) IMPORTANT:
    // Remove unsafe for the non-mut version 

    /// Iterate over the slots for the given id, yielding references to the items
    /// # Safety Figure out why the borrow checker complains
    pub fn item_slots_iter<'a, 'slots, T: 'a>(
        &'a self,
        id: Id,
        slots: &'slots [T],
    ) -> impl Iterator<Item = (usize, &'slots T)> + '_ {
        let slots_ptr = slots.as_ptr();
        self.indices_iter(id)
            .map(move |i| (i, unsafe { &*slots_ptr.add(i) }))
    }

    /// Iterate over the slots for the given id, yielding mutable references to the items
    /// # Safety
    /// Assumes that there are no duplicate slots for the given id
    pub unsafe fn item_slots_iter_mut<'a, 'slots, T: 'a>(
        &'a mut self,
        id: Id,
        slots: &'slots mut [T],
    ) -> impl Iterator<Item = (usize, &'slots mut T)> + '_ {
        let slots_ptr = slots.as_mut_ptr();
        self.indices_iter(id)
            .map(move |i| (i, unsafe { &mut *slots_ptr.add(i) }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_map() {
        let mut ix = IdIndexMap::<u32>::default();

        // Insert 3 ids with slots, 2 duplicates
        ix.insert(0, 0);
        ix.insert(0, 1);
        ix.insert(1, 2);

        // Assert ids
        assert_eq!(ix.indices(&0), Some([0, 1].as_slice()));
        assert_eq!(ix.indices(&1), Some([2].as_slice()));

        // Swap slot 0 and 2
        ix.update(&0, 0, 2).unwrap();
        ix.update(&1, 2, 0).unwrap();

        // There should be no item with id 1 at slot 2 now
        assert!(ix.update(&1, 2, 0).is_err());
        // No item with id 2
        assert!(ix.update(&2, 2, 0).is_err());

        // Assert ids
        assert_eq!(ix.indices(&0), Some([2, 1].as_slice()));
        assert_eq!(ix.indices(&1), Some([0].as_slice()));

        // Remove an item
        ix.remove(0, 2);
        assert_eq!(ix.indices(&0), Some([1].as_slice()));

        // Entry should be removed when count is zero
        ix.remove(0, 1);
        assert_eq!(ix.0.get(&0), None);
    }

    #[test]
    fn sorted_slots() {
        let mut ix = IdIndexMap::<u32>::default();

        ix.insert(0, 3);
        ix.insert(0, 2);
        ix.insert(0, 1);
        ix.insert(0, 4);

        assert_eq!(ix.indices(&0), Some([1, 2, 3, 4].as_slice()));
    }
}
