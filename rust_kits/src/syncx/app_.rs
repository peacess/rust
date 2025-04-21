pub struct StaticBox<T> {
    ptr: Box<T>,
}

impl<T> StaticBox<T> {
    pub fn new(t: T) -> Self {
        Self::from_box(Box::new(t))
    }

    #[inline]
    pub fn from_box(t: Box<T>) -> Self {
        Self { ptr: t }
    }

    #[inline]
    pub fn from_raw(raw: &'static T) -> Self {
        Self {
            ptr: unsafe { Box::from_raw(raw as *const _ as *mut T) },
        }
    }
    #[inline]
    pub fn leak<'a>(b: Self) -> &'a mut T {
        Box::leak(b.ptr)
    }
}

pub struct StaticPointer<T> {
    ptr: *mut T,
}

impl<T> StaticPointer<T> {
    pub fn new(t: T) -> Self {
        Self::from_box(Box::new(t))
    }

    #[inline]
    pub fn from_box(t: Box<T>) -> Self {
        Self { ptr: Box::into_raw(t) }
    }
    #[inline]
    pub fn from_raw(raw: &'static T) -> Self {
        Self {
            ptr: unsafe { Box::leak(Box::from_raw(raw as *const _ as *mut T)) },
        }
    }

    #[inline]
    pub fn leak<'a>(b: Self) -> &'a mut T {
        unsafe { &mut (*b.ptr) }
    }
}

impl<T> Drop for StaticPointer<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}
