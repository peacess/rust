use std::sync::{atomic, atomic::AtomicBool};

trait OrderRelaxed {
    type Item;
    fn get_relaxed(&self) -> Self::Item;

    fn set_relaxed(&self, v: Self::Item);
}

impl OrderRelaxed for AtomicBool {
    type Item = bool;
    fn get_relaxed(&self) -> Self::Item {
        self.load(atomic::Ordering::Relaxed)
    }

    fn set_relaxed(&self, v: Self::Item) {
        self.store(v, atomic::Ordering::Relaxed)
    }
}
