use crate::syncx::{Tasks, ThreadTask};

pub struct ThreadOption<T> {
    inner: ThreadTask<Option<T>>,
}

unsafe impl<T: Sync> Sync for ThreadOption<T> {}

impl<T> Tasks<T> for ThreadOption<T> {
    type InterType = Option<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: ThreadTask::with_capacity(capacity),
        }
    }

    fn push(&self, task: T) {
        self.inner.push(Some(task));
    }

    fn pushes(&self, new_tasks: Vec<T>) {
        let new_tasks = new_tasks.into_iter().map(|it| Some(it)).collect();
        self.inner.pushes(new_tasks);
    }

    fn take_tasks(&self) -> Vec<Self::InterType> {
        self.inner.take_tasks()
    }

    fn swap_tasks(&self, t: &mut Vec<Self::InterType>) {
        self.inner.swap_tasks(t);
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn sync_run<F: Fn(Vec<T>) -> bool>(&self, _handle: F) {
        self.inner.sync_run(|tasks| {
            let mut exit = false;
            let vec_tasks = {
                let mut vec = Vec::with_capacity(tasks.len());
                for task in tasks {
                    match task {
                        None => {
                            exit = true;
                        }
                        Some(data) => {
                            vec.push(data);
                        }
                    }
                }
                vec
            };
            let re = _handle(vec_tasks);
            exit || re
        });
    }

    fn stop(&self) {
        self.push_none();
    }
}

impl<T> ThreadOption<T> {
    pub fn push_none(&self) {
        self.inner.push(None);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::syncx::{Tasks, ThreadOption};

    #[test]
    fn test_thread_option() {
        let thread_option = Arc::new(ThreadOption::<u32>::with_capacity(10));
        // //just for test
        let sum_data = Arc::new(std::sync::atomic::AtomicU32::new(0));

        let thread_task_clone = thread_option.clone();
        let sum_data_clone = sum_data.clone();
        let join_handle = std::thread::spawn(move || {
            thread_task_clone.sync_run(|tasks| {
                for item in tasks {
                    let data = item;
                    {
                        sum_data_clone.fetch_add(data, std::sync::atomic::Ordering::Relaxed);
                    }
                }
                false
            });
        });
        thread_option.push(1);
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(sum_data.load(std::sync::atomic::Ordering::Relaxed), 1);
        thread_option.push(2);
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(sum_data.load(std::sync::atomic::Ordering::Relaxed), 3);
        thread_option.pushes(vec![3, 4]);
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(sum_data.load(std::sync::atomic::Ordering::Relaxed), 10);
        thread_option.stop();
        join_handle.join().unwrap();
    }
}
