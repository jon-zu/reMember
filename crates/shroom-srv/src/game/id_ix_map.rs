use std::collections::{hash_map::Entry, HashMap};

use smallvec::SmallVec;

use super::inventory::{InvError, InvResult};

pub const SMALL_INDEX_CAP: usize = 8;
pub type IndexVec<T> = SmallVec<[T; SMALL_INDEX_CAP]>;

#[derive(Debug)]
pub struct IdIndexMap<Id, Ix>(HashMap<Id, IndexVec<Ix>>);

impl<Id, Ix> Default for IdIndexMap<Id, Ix> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<Id: Eq + std::hash::Hash + Copy, Ix: Ord + Copy> IdIndexMap<Id, Ix> {
    /// Insert a slot index for the given id
    pub fn insert(&mut self, id: Id, slot: Ix) {
        let ix = self.0.entry(id).or_default();

        // Find insert position
        let pos = ix.iter().position(|&s| s > slot).unwrap_or(ix.len());
        ix.insert(pos, slot);
    }

    pub fn remove(&mut self, id: Id, slot: Ix) {
        if let Entry::Occupied(mut entry) = self.0.entry(id) {
            let pos = entry.get().iter().position(|&s| s == slot).unwrap();
            entry.get_mut().remove(pos);
            if entry.get().is_empty() {
                entry.remove();
            }
        }
    }

    pub fn update(&mut self, id: &Id, old_slot: Ix, new_slot: Ix) -> InvResult<()> {
        //TODO use a proper error
        let ix = self.0.get_mut(id).ok_or(InvError::EmptySlot(1337))?;

        let pos = ix
            .iter()
            .position(|slot| *slot == old_slot)
            .ok_or(InvError::EmptySlot(1337))?;

        ix[pos] = new_slot;
        // Resort from the inserted slot
        // TODO: in theory this be done with insert sort
        // from the inserted position
        ix.sort_unstable();
        Ok(())
    }

    pub fn contains_id(&self, id: &Id) -> bool {
        self.0.contains_key(id)
    }

    pub fn indices(&self, id: &Id) -> Option<&[Ix]> {
        self.0.get(id).map(|s| s.as_slice())
    }

    pub fn indices_iter(&self, id: &Id) -> impl Iterator<Item = Ix> + '_ {
        self.indices(id).into_iter().flatten().cloned()
    }

    pub fn indices_cloned(&self, id: &Id) -> Option<IndexVec<Ix>> {
        self.0.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_map() {
        let mut ix = IdIndexMap::<u32, usize>::default();

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
        assert_eq!(ix.indices(&0), Some([1, 2].as_slice()));
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
        let mut ix = IdIndexMap::<u32, usize>::default();

        ix.insert(0, 3);
        ix.insert(0, 2);
        ix.insert(0, 1);
        ix.insert(0, 4);

        assert_eq!(ix.indices(&0), Some([1, 2, 3, 4].as_slice()));
    }
}
