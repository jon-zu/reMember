use std::marker::PhantomData;

/// Non-threadsafe broadcast channel,
/// for sending messages to multiple receivers.

struct Slot<T> {
    data: Option<T>,
    remaining: usize,
    pos: u64,
}

impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            remaining: Default::default(),
            pos: Default::default(),
        }
    }
}

pub struct BroadcastChannel<T> {
    slots: Vec<Slot<T>>,
    ix: usize,
    pos: u64,
    rx: usize
}

pub struct BroadcastReceiver<T> {
    pos: u64,
    ix: usize,
    _t: PhantomData<T>,
}

impl<T> BroadcastChannel<T> {
    pub fn new() -> Self {
        Self {
            slots: Vec::from_iter((0..128).map(|_| Slot::default())),
            pos: 0,
            ix: 0,
            rx: 0,
        }
    }

    pub fn send(&mut self, msg: T) {
        self.pos += 1;
        self.ix = self.next_ix(self.ix);
        self.slots[self.ix] = Slot {
            data: Some(msg),
            remaining: self.rx,
            pos: self.pos,
        }; 
    }

    fn next_ix(&self, ix: usize) -> usize {
        (ix + 1) % self.slots.len()
    }

    pub fn receiver(&mut self) -> BroadcastReceiver<T> {
        self.rx += 1;
        BroadcastReceiver { pos: self.pos, ix: self.ix,  _t: PhantomData } 
    }
}


impl<T: Clone> BroadcastReceiver<T> {
    pub fn recv(&mut self, ch: &mut BroadcastChannel<T>) -> Option<T> {
        let mut ix = ch.ix;
        while ch.slots[ix].pos <= self.pos {
            ix = (ix + 1) % ch.slots.len();
            if ix == ch.ix {
                return None;
            }
        }
        let slot = &mut ch.slots[ix];
        slot.remaining -= 1;
        if slot.remaining == 0 {
            ch.pos = slot.pos;
            ch.rx -= 1;
        }
        slot.data.take()
    }
}