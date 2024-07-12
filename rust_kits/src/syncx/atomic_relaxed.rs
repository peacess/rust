use crate::syncx::AtomicT;
use core::sync::atomic;
use std::sync::Arc;

#[derive(Debug)]
pub struct AtomicRelaxed<T>(pub T);

impl<T: bytemuck::NoUninit> AtomicRelaxed<atomic_g::Atomic<T>> {
    pub fn new(t: T) -> Self {
        Self(atomic_g::Atomic::new(t))
    }
    pub fn get(&self) -> T {
        self.0.load(atomic::Ordering::Relaxed)
    }

    pub fn set(&self, t: T) {
        self.0.store(t, atomic::Ordering::Relaxed);
    }

    pub fn get_ordering(&self, ordering: atomic::Ordering) -> T {
        self.0.load(ordering)
    }

    pub fn set_ordering(&self, t: T, ordering: atomic::Ordering) {
        self.0.store(t, ordering);
    }
}

impl AtomicRelaxed<atomic::AtomicBool> {
    pub fn new(t: bool) -> Self {
        Self(atomic::AtomicBool::new(t))
    }

    pub fn get(&self) -> bool {
        self.0.load(atomic::Ordering::Relaxed)
    }

    pub fn set(&self, t: bool) {
        self.0.store(t, atomic::Ordering::Relaxed);
    }
    pub fn get_ordering(&self, ordering: atomic::Ordering) -> bool {
        self.0.load(ordering)
    }

    pub fn set_ordering(&self, t: bool, ordering: atomic::Ordering) {
        self.0.store(t, ordering);
    }
}

macro_rules! std_atomic {
    ($t:ty,$v:ident) => {
        impl AtomicRelaxed<$t> {
            pub fn new(t: $v) -> Self {
                Self(<$t>::new(t))
            }

            pub fn get(&self) -> $v {
                self.0.load(atomic::Ordering::Relaxed)
            }

            pub fn set(&self, t: $v) {
                self.0.store(t, atomic::Ordering::Relaxed);
            }
            pub fn get_ordering(&self, ordering: atomic::Ordering) -> $v {
                self.0.load(ordering)
            }

            pub fn set_ordering(&self, t: $v, ordering: atomic::Ordering) {
                self.0.store(t, ordering);
            }
        }
    };
}
std_atomic!(atomic::AtomicI8, i8);
std_atomic!(atomic::AtomicI16, i16);
std_atomic!(atomic::AtomicI32, i32);
std_atomic!(atomic::AtomicI64, i64);
std_atomic!(atomic::AtomicIsize, isize);
std_atomic!(atomic::AtomicU8, u8);
std_atomic!(atomic::AtomicU16, u16);
std_atomic!(atomic::AtomicU32, u32);
std_atomic!(atomic::AtomicU64, u64);
std_atomic!(atomic::AtomicUsize, usize);

impl<T> AtomicRelaxed<atomic::AtomicPtr<T>> {
    pub fn new(t: *mut T) -> Self {
        Self(atomic::AtomicPtr::new(t))
    }

    pub fn get(&self) -> *mut T {
        self.0.load(atomic::Ordering::Relaxed)
    }

    pub fn set(&self, t: *mut T) {
        self.0.store(t, atomic::Ordering::Relaxed);
    }
    pub fn get_ordering(&self, ordering: atomic::Ordering) -> *mut T {
        self.0.load(ordering)
    }

    pub fn set_ordering(&self, t: *mut T, ordering: atomic::Ordering) {
        self.0.store(t, ordering);
    }
}

impl<T> AtomicRelaxed<AtomicT<T>> {
    pub fn new(t: T) -> Self {
        Self(AtomicT::new(t))
    }

    pub fn get(&self) -> Option<Arc<T>> {
        self.0.load(atomic::Ordering::Relaxed)
    }

    pub fn set(&self, t: Arc<T>) {
        self.0.replace(t, atomic::Ordering::Relaxed);
    }
    pub fn get_ordering(&self, ordering: atomic::Ordering) -> Option<Arc<T>> {
        self.0.load(ordering)
    }

    pub fn set_ordering(&self, t: Arc<T>, ordering: atomic::Ordering) {
        self.0.replace(t, ordering);
    }
}

#[cfg(test)]
mod test {
    use crate::syncx::AtomicRelaxed;

    #[derive(Clone, Copy)]
    enum Status {
        None = 0,
        Ok = 1,
    }

    unsafe impl bytemuck::NoUninit for Status {}

    #[test]
    fn test() {
        let _ = AtomicRelaxed::<atomic_g::Atomic<Status>>::new(Status::None);
        let _ = AtomicRelaxed::<atomic_g::Atomic<Status>>::new(Status::Ok);
    }
}
