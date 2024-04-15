use std::{marker::PhantomData, time::Duration};

use bitflags::Flags;

use crate::{util::DelayQueue, Instant};

pub type BuffEpoch = usize;

pub trait Buff {
    fn get_expiration(&self) -> Option<Instant>;
    fn is_expired(&self, now: Instant) -> bool {
        self.get_expiration().is_some_and(|t| t <= now)
    }

    fn get_duration(&self, now: Instant) -> Option<Duration> {
        self.get_expiration().map(|t| {
            t.checked_duration_since(now)
                .unwrap_or_else(|| Duration::from_secs(0))
        })
    }
}

pub trait MappedBuff<B: Buff, F: BuffFlag> {
    fn flag() -> F;

    fn from_buff(buff: &B) -> Self;
    fn to_buff(self) -> B;
}

pub trait BuffFlag: Clone + bitflags::Flags {
    fn get_key(&self) -> usize;
    fn from_key(key: usize) -> Self;
}

pub trait BuffStorage {
    type Buff: Buff;
    type Flag: BuffFlag;

    fn insert_buff(&mut self, key: usize, epoch: BuffEpoch, buff: Self::Buff);
    fn remove_buff(&mut self, key: usize, epoch: BuffEpoch) -> bool;

    fn get_buff(&self, key: usize) -> Option<&Self::Buff>;
    fn get_buff_mut(&mut self, key: usize) -> Option<&mut Self::Buff>;
}

#[derive(Debug)]
pub struct BuffManager<T: BuffStorage> {
    storage: T,
    update_mask: T::Flag,
    buff_mask: T::Flag,
    expiring_buffs: DelayQueue<(usize, BuffEpoch)>,
    epoch: BuffEpoch,
}

impl<T: BuffStorage + Default> Default for BuffManager<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: BuffStorage> BuffManager<T> {
    pub fn new(storage: T) -> Self {
        Self {
            update_mask: T::Flag::empty(),
            buff_mask: T::Flag::empty(),
            expiring_buffs: DelayQueue::new(),
            storage,
            epoch: 0,
        }
    }

    fn set_buff_flag(&mut self, flag: T::Flag, dur: Option<Duration>, t: Instant) {
        if let Some(dur) = dur {
            self.expiring_buffs.push((flag.get_key(), self.epoch), t + dur);
        }
        self.update_mask.insert(flag.clone());
        self.buff_mask.insert(flag);
        self.epoch += 1;
    }

    fn mark_buff_updated(&mut self, flag: T::Flag) {
        self.update_mask.insert(flag);
    }

    pub fn storage(&self) -> &T {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut T {
        &mut self.storage
    }

    pub fn set_buff(&mut self, flag: T::Flag, t: Instant, buff: T::Buff) {
        let dur = buff.get_duration(t);
        self.storage.insert_buff(flag.get_key(), self.epoch, buff);
        self.set_buff_flag(flag, dur, t);
    }

    pub fn update(&mut self, flag: T::Flag, f: impl FnOnce(&mut T::Buff) -> bool) {
        // TODO handle buff extension
        if self.buff_mask.contains(flag.clone()) {
            let buff = self.storage.get_buff_mut(flag.get_key()).unwrap();
            if f(buff) {
                self.mark_buff_updated(flag);
            }
        }
    }

    pub fn set_mapped<U: MappedBuff<T::Buff, T::Flag>>(&mut self, buff: U, t: Instant) {
        self.set_buff(U::flag(), t, buff.to_buff());
    }

    pub fn update_mapped<U: MappedBuff<T::Buff, T::Flag>>(
        &mut self,
        f: impl FnOnce(&mut U) -> bool,
    ) {
        self.update(U::flag(), |b| {
            let mut mapped = U::from_buff(b);
            if f(&mut mapped) {
                *b = mapped.to_buff();
                true
            } else {
                false
            }
        });
    }

    pub fn force_update(&mut self, flag: T::Flag, f: impl FnOnce(&mut T::Buff)) {
        self.update(flag, |b| {
            f(b);
            true
        });
    }

    pub fn get_buff(&self, flag: &T::Flag) -> Option<&T::Buff> {
        self.storage.get_buff(flag.get_key())
    }

    pub fn get_buff_mut(&mut self, flag: &T::Flag) -> Option<&mut T::Buff> {
        self.storage.get_buff_mut(flag.get_key())
    }


    pub fn buff_flags(&self) -> T::Flag {
        self.buff_mask.clone()
    }


    pub fn take_update_flags(&mut self) -> Option<T::Flag> {
        if self.update_mask.is_empty() {
            return None;
        }

        Some(std::mem::replace(
            &mut self.update_mask,
            T::Flag::empty(),
        ))
    }

    pub fn tick_expiration(&mut self, t: Instant) -> Option<T::Flag> {
        let expire_mask = self.expiring_buffs.drain_expired(t).fold(
            T::Flag::empty(),
            |mut acc, (key, epoch)| {
                if self.storage.remove_buff(key, epoch) {
                    let flag = T::Flag::from_key(key);
                    self.buff_mask.remove(flag.clone());
                    acc.insert(flag);
                }
                acc
            },
        );

        if expire_mask.is_empty() {
            return None;
        }
        Some(expire_mask)
    }
}

#[derive(Debug)]
pub struct BuffFlagStorage<const N: usize, K, B> {
    buffs: [(BuffEpoch, B); N],
    _k: PhantomData<K>,
}

impl<const N: usize, K, Buff: Default> Default for BuffFlagStorage<N, K, Buff> {
    fn default() -> Self {
        Self {
            buffs: [(); N].map(|()| Default::default()),
            _k: PhantomData,
        }
    }
}

impl<const N: usize, F: BuffFlag, B: Buff> BuffStorage for BuffFlagStorage<N, F, B>
{
    type Buff = B;
    type Flag = F;

    fn insert_buff(&mut self, key: usize, epoch: BuffEpoch, buff: Self::Buff) {
        self.buffs[key] = (epoch, buff);
    }

    fn remove_buff(&mut self, key: usize, epoch: BuffEpoch) -> bool {
        self.buffs[key].0 == epoch
    }

    fn get_buff(&self, key: usize) -> Option<&Self::Buff> {
        self.buffs.get(key).map(|x| &x.1)
    }

    fn get_buff_mut(&mut self, key: usize) -> Option<&mut Self::Buff> {
        self.buffs.get_mut(key).map(|x| &mut x.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{time::clock::Ticks, Clock};

    use super::{BuffManager, *};

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct B(pub char, pub Instant);


    impl Buff for B {
        fn get_expiration(&self) -> Option<Instant> {
            Some(self.1)
        }
    }

    bitflags::bitflags! {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
        struct KeyFlags: u8 {
            const A = 1;
            const B = 1 << 1;
            const C = 1 << 2;
        }
    }

    impl BuffFlag for KeyFlags {
        fn get_key(&self) -> usize {
            self.bits().trailing_zeros() as usize
        }

        fn from_key(key: usize) -> Self {
            Self::from_bits_truncate(1 << key as u8)
        }
    }


    type BufMan = BuffManager<BuffFlagStorage<3, KeyFlags, B>>;

    #[test]
    fn buffs() {
        let clock = Clock::default();
        let mut buf_man = BufMan::default();

        let t = clock.time();

        // Set buff
        buf_man.set_buff(KeyFlags::A, t, B('1', t + Ticks(1)));
        assert_eq!(buf_man.take_update_flags(), Some(KeyFlags::A));
        assert!(buf_man.take_update_flags().is_none());
        assert!(buf_man.tick_expiration(t).is_none());
        assert_eq!(buf_man.buff_flags(), KeyFlags::A);


        // Update buff
        assert_eq!(buf_man.get_buff(&KeyFlags::A).unwrap().0, '1');
        buf_man.update(KeyFlags::A, |B(c, _)| {
            *c = '2';
            true
        });
        assert_eq!(buf_man.get_buff(&KeyFlags::A).unwrap().0, '2');
        assert_eq!(buf_man.take_update_flags(), Some(KeyFlags::A));
        assert!(buf_man.take_update_flags().is_none());

        // Tick expiration
        let t = t + Ticks(1);
        assert_eq!(buf_man.tick_expiration(t), Some(KeyFlags::A));
        assert!(buf_man.tick_expiration(t).is_none());
        assert!(buf_man.buff_flags().is_empty());

    }
}
