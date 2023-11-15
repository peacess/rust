use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};

/// AtomicT与　AtomicPtr：
/// AtomicPtr：
/// 1. 无锁线程安全
/// 2. 它不会管理其中的raw指针
/// AtomicT 优点：　
/// 1. 无锁线程安全（free lock）
/// 2. 可以正确释放Ｔ对象 (free t)
/// 3. 当一个对象被替换后，原来的对象还可以正常使用，且内存也会正常释放
///
#[derive(Debug)]
pub struct AtomicT<T>(AtomicPtr<Arc<T>>);

impl<T> Drop for AtomicT<T> {
    fn drop(&mut self) {
        //对象本身释放时，释放自己管理的指针
        let data = self.0.swap(ptr::null_mut(), Ordering::SeqCst);
        if !data.is_null() {
            unsafe {
                let _ = Box::from_raw(data);
            }
        }
    }
}

impl<T> AtomicT<T> {
    pub fn new(value: T) -> Self {
        let ptr_data = Box::into_raw(Box::new(Arc::new(value)));
        AtomicT(AtomicPtr::new(ptr_data))
    }
    //这个方法的名字比较default　更明确，所以增加此方法
    pub fn null() -> AtomicT<T> {
        AtomicT(AtomicPtr::new(ptr::null_mut()))
    }

    #[inline]
    pub fn load(&self, order: Ordering) -> Option<Arc<T>> {
        let temp = self.0.load(order);
        if temp.is_null() {
            return None;
        } else {
            return Some(unsafe { (*temp).clone() });
        }
    }

    #[inline]
    pub fn replace(&self, data: Arc<T>, order: Ordering) {
        let ptr_data = Box::into_raw(Box::new(data));
        let t = self.0.swap(ptr_data, order);
        if !t.is_null() {
            unsafe { let _ = Box::from_raw(t); }
        }
    }

    #[inline]
    pub fn swap(&self, data: Arc<T>, order: Ordering) -> Option<Arc<T>> {
        let ptr_data = Box::into_raw(Box::new(data));
        let temp = self.0.swap(ptr_data, order);
        if temp.is_null() {
            return None;
        } else {
            let t = unsafe { Box::from_raw(temp) };
            return Some(t.as_ref().clone());
        }
    }
}

impl<T> Default for AtomicT<T> {
    fn default() -> Self {
        AtomicT::null()
    }
}

/// the pointer must be in heap
impl<T> From<Arc<T>> for AtomicT<T> {
    fn from(data: Arc<T>) -> Self {
        let ptr_data = Box::into_raw(Box::new(data));
        AtomicT(AtomicPtr::new(ptr_data))
    }
}

// AtomicPtr是　Ｓend 与　Sync的所以　AtomicＴ也是，就不用显示实现了
// unsafe impl<T> Send for AtomicＴ<T>{}
// unsafe impl<T> Sync for AtomicＴ<T>{}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    use crate::syncx::AtomicT;

    /// 在Cargo.toml目录下，运行　“cargo valgrind test --color=always --lib syncx::atomic_t::tests::drop_self_test”，测试程序是否有内存问题
    /// 注：　如果没有安装　valgrind 需要先运行：　cargo install cargo-valgrind
    #[test]
    fn drop_self_test() {
        {//case one
            let p = AtomicT::new(1);
            mem::drop( p);
        }
        {// 各种操作是否有内存问题
            let p = AtomicT::new(1);
            let value_load = p.load(Ordering::SeqCst).expect("");
            assert_eq!(1, *value_load.as_ref());
            p.replace(Arc::new(2), Ordering::SeqCst);
            let value_load = p.load(Ordering::SeqCst).expect("");
            assert_eq!(2, *value_load.as_ref());
            let value_swap = p.swap(Arc::new(3), Ordering::SeqCst).expect("");
            assert_eq!(2, *value_swap.as_ref());
            let value_load = p.load(Ordering::SeqCst).expect("");
            assert_eq!(3, *value_load.as_ref());
        }
    }
}