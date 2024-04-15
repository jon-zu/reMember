use std::{collections::HashMap, marker::PhantomData, time::Duration};

use shroom_meta::{
    buffs::{
        char::{CharBuff, CharBuffStat, CharBuffStorage},
        keys::CharBuffKey,
        BuffKey,
    },
    id::BuffId,
};

use shroom_proto95::game::user::secondary_stats::CharSecondaryStatFlags;
use shroom_srv::{util::DelayQueue, GameTime};

mod pkt;
pub use pkt::{CharBuffPacket, CharBuffRemotePacket};


#[derive(Debug, Default, Clone, Copy)]
struct ExpData {
    expiration: GameTime,
    gen: usize,
    extended: bool
}

impl ExpData {
    pub fn new(exp: GameTime, gen: usize) -> Self {
        Self {
            expiration: exp,
            gen,
            extended: false,
        }
    }

    pub fn is_expired(&self, t: GameTime) -> bool {
        self.expiration <= t
    }

    pub fn is_extended(&mut self) -> bool {
        if self.extended {
            self.extended = false;
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct BuffExpirations<const N: usize, F> {
    expirations: [ExpData; N],
    q: DelayQueue<(F, usize)>,
    gen: usize,
    _f: PhantomData<F>,
}

impl<const N: usize, F: BuffKey> BuffExpirations<N, F> {
    fn next_gen(&mut self) -> usize {
        let gen = self.gen;
        self.gen += 1;
        gen
    }


    pub fn insert(&mut self, ix: F, exp: GameTime) {
        let gen = self.next_gen();
        self.expirations[ix.as_index()] = ExpData::new(exp, gen);
        self.q.push((ix, gen), exp);
    }

    pub fn extend(&mut self, ix: F, exp: GameTime) {
        let v = &mut self.expirations[ix.as_index()];
        v.expiration = exp;
        v.extended = true;
    }

    pub fn expiration_dur(&self, ix: F, t: GameTime) -> Duration {
        self.expirations[ix.as_index()]
            .expiration
            .checked_duration_since(t)
            .unwrap_or(Duration::ZERO)
    }

    pub fn is_expired(&self, ix: F, t: GameTime) -> bool {
        self.expirations[ix.as_index()].is_expired(t)
    }

    pub fn next_expired(&mut self, t: GameTime) -> Option<F> {
        while let Some((buff_ix, gen)) = self.q.pop(t) {
            let ix = buff_ix.as_index();
            let e = &mut self.expirations[ix];
            if e.gen != gen {
                continue;
            }


            if !e.is_expired(t) {
                // If the buff got extended, reset the flag
                // and re-enque
                let e = &mut self.expirations[ix];
                if e.is_extended() {
                    self.q.push((buff_ix, e.gen), e.expiration);
                    continue;
                }
            }

            return Some(buff_ix);
        }

        None
    }

    pub fn mark_removed(&mut self, ix: F) {
        // Invalidate the generation
        self.expirations[ix.as_index()].gen += 1;
    }
}

impl<const N: usize, F: BuffKey> Default for BuffExpirations<N, F> {
    fn default() -> Self {
        Self {
            expirations: [ExpData::default(); N],
            q: DelayQueue::new(),
            gen: 1,
            _f: PhantomData,
        }
    }
}

pub trait ApplyBuff {
    fn apply_buff(&self, t: GameTime, buffs: &mut CharBuffs);
}

impl<T: CharBuffStat + Clone> ApplyBuff for CharBuff<T> {
    fn apply_buff(&self, t: GameTime, buffs: &mut CharBuffs) {
        buffs.set(t, self.clone());
    }
}

impl<T: CharBuffStat + Clone, U: CharBuffStat + Clone> ApplyBuff for (CharBuff<T>, CharBuff<U>) {
    fn apply_buff(&self, t: GameTime, buffs: &mut CharBuffs) {
        self.0.apply_buff(t, buffs);
        self.1.apply_buff(t, buffs);
    }
}

#[derive(Debug)]
pub struct CharBuffs {
    buff_flag: CharSecondaryStatFlags,
    remove_flag: CharSecondaryStatFlags,
    update_flag: CharSecondaryStatFlags,
    storage: CharBuffStorage,
    exp: BuffExpirations<128, CharBuffKey>,
    skills: HashMap<BuffId, CharSecondaryStatFlags>,
}

impl Default for CharBuffs {
    fn default() -> Self {
        Self {
            buff_flag: CharSecondaryStatFlags::empty(),
            remove_flag: CharSecondaryStatFlags::empty(),
            update_flag: CharSecondaryStatFlags::empty(),
            storage: CharBuffStorage::default(),
            exp: BuffExpirations::default(),
            skills: HashMap::new(),
        }
    }
}

impl CharBuffs {
    pub fn new() -> Self {
        Self::default()
    }

    fn remove_buff_from_skill(&mut self, id: BuffId, flag: CharSecondaryStatFlags) {
        match self.skills.entry(id) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                e.get_mut().remove(flag);
                if e.get().is_empty() {
                    e.remove();
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {}
        }
    }

    pub fn cancel_all_skills(&mut self) {
        for (_, flags) in self.skills.drain() {
            for flag in flags.iter() {
                self.buff_flag.remove(flag.clone());
                self.remove_flag.insert(flag.clone());
                self.exp.mark_removed(CharBuffKey::from_flag(flag.bits()));
            }
        }
    }

    pub fn cancel_by_id(&mut self, id: BuffId) {
        if let Some(flag) = self.skills.remove(&id) {
            for flag in flag.iter() {
                self.buff_flag.remove(flag.clone());
                self.remove_flag.insert(flag.clone());
                self.exp.mark_removed(CharBuffKey::from_flag(flag.bits()));
            }
        }
    }

    pub fn set<T: CharBuffStat>(&mut self, t: GameTime, buff: CharBuff<T>) {
        let f = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());

        // Overwrite the buff
        if let Some(old) = self.get::<T>() {
            self.remove_buff_from_skill(old.id, f.clone())
        }

        let id = buff.id;
        self.buff_flag.insert(f.clone());
        self.update_flag.insert(f.clone());
        self.remove_flag.remove(f.clone());
        self.exp.insert(T::KEY, t + buff.dur);
        self.storage.set(buff);

        // Add to skill
        self.skills
            .entry(id)
            .and_modify(|e| e.insert(f.clone()))
            .or_insert(f);
    }

    pub fn get<T: CharBuffStat>(&self) -> Option<&CharBuff<T>> {
        let f = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());
        self.buff_flag.contains(f).then_some(T::get(&self.storage))
    }

    pub fn remove<T: CharBuffStat>(&mut self) {
        let f = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());
        self.remove_flag.insert(f.clone());

        let id = T::get(&self.storage).id;
        self.remove_buff_from_skill(id, f.clone());
        self.buff_flag.remove(f);
    }

    pub fn update<T: CharBuffStat>(&mut self, f: impl Fn(&mut T) -> bool) {
        let fl = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());
        if self.buff_flag.contains(fl.clone()) {
            let buff = T::get_mut(&mut self.storage);
            if f(&mut buff.data) {
                self.update_flag.insert(fl);
            }
        }
    }

    pub fn update_extend<T: CharBuffStat>(&mut self, t: GameTime, f: impl Fn(&mut T) -> bool) {
        let fl = CharSecondaryStatFlags::from_bits_truncate(T::KEY.flag());
        if self.buff_flag.contains(fl.clone()) {
            let buff = T::get_mut(&mut self.storage);
            if f(&mut buff.data) {
                self.update_flag.insert(fl);
                self.exp.extend(T::KEY, t + buff.dur);
            }
        }
    }

    pub fn update_expirations(&mut self, t: GameTime) -> CharSecondaryStatFlags {
        while let Some(exp) = self.exp.next_expired(t) {
            let fl = CharSecondaryStatFlags::from_bits_truncate(exp.flag());
            self.buff_flag.remove(fl.clone());
            self.remove_flag.insert(fl);
        }

        std::mem::take(&mut self.remove_flag)
    }

    pub fn take_updated(&mut self) -> CharSecondaryStatFlags {
        let f = std::mem::take(&mut self.update_flag);
        f.intersection(self.buff_flag.clone())
    }
}


#[cfg(test)]
mod tests {
    use shroom_srv::time::clock::Ticks;

    use super::*;

    #[test]
    fn expirations() {
        const T1: Ticks = Ticks(1);
        let mut exp = BuffExpirations::<128, CharBuffKey>::default();
        let t = GameTime::default();
        
        // Simple Expiration
        exp.insert(CharBuffKey::Pad, t + T1); 
        assert_eq!(exp.next_expired(t), None);
        assert_eq!(exp.next_expired(t + T1), Some(CharBuffKey::Pad));

        // Extension
        exp.insert(CharBuffKey::Pad, t + T1);
        exp.extend(CharBuffKey::Pad, t + Ticks(5));
        // Buff should get extended here
        assert_eq!(exp.next_expired(t + T1), None);
        assert_eq!(exp.next_expired(t + Ticks(4)), None);
        assert_eq!(exp.next_expired(t + Ticks(5)), Some(CharBuffKey::Pad));
        assert_eq!(exp.next_expired(t + Ticks(5)), None);


        // Removal
        exp.insert(CharBuffKey::Pad, t + T1);
        exp.mark_removed(CharBuffKey::Pad);
        assert_eq!(exp.next_expired(t + T1), None);
    }
}