#[derive(Debug)]
pub struct AtomicGetSet<T>(pub T);

impl<T: bytemuck::NoUninit> AtomicGetSet<atomic::Atomic<T>> {
    pub fn new(t: T) -> Self {
        Self(atomic::Atomic::new(t))
    }
    pub fn get(&self) -> T {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set(&self, t: T) {
        self.0.store(t, std::sync::atomic::Ordering::Relaxed);
    }
}

impl AtomicGetSet<std::sync::atomic::AtomicBool> {
    pub fn new(t: bool) -> Self {
        Self(std::sync::atomic::AtomicBool::new(t))
    }

    pub fn get(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set(&self, t: bool) {
        self.0.store(t, std::sync::atomic::Ordering::Relaxed);
    }
}

macro_rules! std_atomic {
    ($t:ty,$v:ident) => {
        impl AtomicGetSet<$t> {
            pub fn new(t: $v) -> Self {
                Self(<$t>::new(t))
            }

            pub fn get(&self) -> $v {
                self.0.load(std::sync::atomic::Ordering::Relaxed)
            }

            pub fn set(&self, t: $v) {
                self.0.store(t, std::sync::atomic::Ordering::Relaxed);
            }
        }
    };
}
std_atomic!(std::sync::atomic::AtomicI8, i8);
std_atomic!(std::sync::atomic::AtomicI16, i16);
std_atomic!(std::sync::atomic::AtomicI32, i32);
std_atomic!(std::sync::atomic::AtomicI64, i64);
std_atomic!(std::sync::atomic::AtomicIsize, isize);
std_atomic!(std::sync::atomic::AtomicU8, u8);
std_atomic!(std::sync::atomic::AtomicU16, u16);
std_atomic!(std::sync::atomic::AtomicU32, u32);
std_atomic!(std::sync::atomic::AtomicU64, u64);
std_atomic!(std::sync::atomic::AtomicUsize, usize);

#[cfg(test)]
mod test {
    use crate::syncx::AtomicGetSet;

    #[derive(Clone, Copy)]
    enum Status{
        None = 0,
        Ok = 1,
    }

    unsafe impl bytemuck::NoUninit for Status{}

    #[test]
    fn test(){
        let _ = AtomicGetSet::<atomic_g::Atomic<Status>>::new(Status::None);
    }
}


