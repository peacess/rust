use std::{cell::RefCell, thread::Thread};

#[derive(Debug)]
pub struct ThreadTask<T> {
    mutex: parking_lot::Mutex<Vec<T>>,
    thread: RefCell<Option<Thread>>,
    capacity: usize,
}

unsafe impl<T: Sync> Sync for ThreadTask<T> {}

impl<T> ThreadTask<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            mutex: parking_lot::Mutex::new(Vec::with_capacity(capacity)),
            thread: RefCell::new(None),
            capacity,
        }
    }

    pub fn push(&self, task: T) {
        let mut tasks = self.mutex.lock();
        tasks.push(task);
        if let Some(t) = self.thread.borrow().as_ref() {
            t.unpark();
        }
    }

    pub fn pushes(&self, new_tasks: Vec<T>) {
        let mut tasks = self.mutex.lock();
        tasks.extend(new_tasks);
        if let Some(t) = self.thread.borrow().as_ref() {
            t.unpark();
        }
    }

    pub fn take_tasks(&self) -> Vec<T> {
        let mut tasks = self.mutex.lock();
        let mut cleaned = Vec::with_capacity(self.capacity);
        std::mem::swap(&mut cleaned, &mut tasks);
        cleaned
    }

    pub fn len(&self) -> usize {
        self.mutex.lock().len()
    }
    pub fn is_empty(&self) -> bool {
        self.mutex.lock().is_empty()
    }

    /// dead loop until the handle return true
    pub fn sync_run<F: Fn(Vec<T>) -> bool>(&self, handle: F) {
        {
            let _tasks = self.mutex.lock();
            if let Ok(mut t) = self.thread.try_borrow_mut() {
                *t = Some(std::thread::current())
            }
        }

        loop {
            let mut local = Vec::with_capacity(self.capacity);
            if local.is_empty() {
                let mut tasks = self.mutex.lock();
                if tasks.is_empty() {
                    drop(tasks);
                    std::thread::park();
                } else {
                    std::mem::swap(&mut local, &mut tasks);
                }
                if local.is_empty() {
                    continue;
                }
            }
            if !local.is_empty() {
                if handle(local) {
                    // set thread is none, because the thread exit
                    if let Ok(mut t) = self.thread.try_borrow_mut() {
                        *t = None
                    }
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::syncx::ThreadTask;

    #[test]
    fn test_thread_task() {
        let thread_task = Arc::new(ThreadTask::<Option<u32>>::with_capacity(10));
        //just for test
        let sum_data = Arc::new(std::sync::atomic::AtomicU32::new(0));

        let thread_task_clone = thread_task.clone();
        let sum_data_clone = sum_data.clone();
        let join_handle = std::thread::spawn(move || {
            thread_task_clone.sync_run(|tasks| {
                for item in tasks {
                    if let Some(data) = item {
                        sum_data_clone.fetch_add(data, std::sync::atomic::Ordering::Relaxed);
                    } else {
                        return true;
                    }
                }
                false
            });
        });
        thread_task.push(Some(1));
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(sum_data.load(std::sync::atomic::Ordering::Relaxed), 1);
        thread_task.push(Some(2));
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(sum_data.load(std::sync::atomic::Ordering::Relaxed), 3);
        thread_task.pushes(vec![Some(3), Some(4)]);
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(sum_data.load(std::sync::atomic::Ordering::Relaxed), 10);
        thread_task.push(None);
        join_handle.join().unwrap();
    }
}
