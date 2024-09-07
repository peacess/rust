use std::collections::VecDeque;

pub struct FastChannel<T> {
    mutex: parking_lot::Mutex<VecDeque<T>>,
    cond: parking_lot::Condvar,
}

impl<T> FastChannel<T> {
    pub fn new() -> Self {
        Self {
            mutex: parking_lot::Mutex::new(VecDeque::new()),
            cond: parking_lot::Condvar::new(),
        }
    }
    pub fn with_capacity(bound: usize) -> Self {
        Self {
            mutex: parking_lot::Mutex::new(VecDeque::with_capacity(bound)),
            cond: parking_lot::Condvar::new(),
        }
    }
    pub fn send(&self, data: T) {
        let mut guard = self.mutex.lock();
        guard.push_front(data);
        self.cond.notify_all();
    }
    pub fn send_not_notify(&self, data: T) {
        let mut guard = self.mutex.lock();
        guard.push_front(data);
    }
    pub fn recv(&self) -> Option<T> {
        let mut guard = self.mutex.lock();
        guard.pop_front()
    }
    pub fn recv_wait(&self) -> T {
        let mut guard = self.mutex.lock();
        loop {
            let v = guard.pop_front();
            match v {
                Some(data) => return data,
                None => {}
            }
            self.cond.wait(&mut guard);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.mutex.lock().is_empty()
    }
    pub fn clear_data(&self) {
        let mut guard = self.mutex.lock();
        guard.clear();
    }

    pub fn len(&self) -> usize {
        self.mutex.lock().len()
    }
}
