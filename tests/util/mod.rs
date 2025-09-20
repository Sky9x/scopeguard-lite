use std::cell::Cell;
use std::thread;

pub struct CheckCallOrder {
    max: u32,
    next: Cell<u32>,
}

impl CheckCallOrder {
    pub fn new(max: u32) -> Self {
        CheckCallOrder {
            max,
            next: Cell::new(0),
        }
    }

    #[track_caller]
    pub fn check(&self, id: u32) {
        if thread::panicking() {
            return;
        }

        let next = self.next.get();
        assert_eq!(id, next);
        self.next.set(next + 1);
    }
}

impl Drop for CheckCallOrder {
    fn drop(&mut self) {
        if thread::panicking() {
            return;
        }

        assert_eq!(self.next.get(), self.max);
    }
}
