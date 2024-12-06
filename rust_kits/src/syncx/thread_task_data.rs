use crate::syncx::{task::TaskData, Tasks, ThreadTask};

pub struct ThreadTaskData<T> {
    inner: ThreadTask<TaskData<T>>,
}

unsafe impl<T: Sync> Sync for ThreadTaskData<T> {}
impl<T> Tasks<T> for ThreadTaskData<T> {
    type InterType = TaskData<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: ThreadTask::with_capacity(capacity),
        }
    }

    fn push(&self, task: T) {
        self.inner.push(TaskData::Data(task));
    }

    fn pushes(&self, new_tasks: Vec<T>) {
        let new_tasks = new_tasks.into_iter().map(|it| TaskData::Data(it)).collect();
        self.inner.pushes(new_tasks);
    }

    fn take_tasks(&self) -> Vec<Self::InterType> {
        self.inner.take_tasks()
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
            let mut nofity = false;
            let vec_tasks = {
                let mut vec = Vec::with_capacity(tasks.len());
                for task in tasks {
                    match task {
                        TaskData::Exit => {
                            exit = true;
                        }
                        TaskData::Data(data) => {
                            vec.push(data);
                        }
                        TaskData::Notify => {
                            nofity = true;
                        }
                    }
                }
                vec
            };
            let re = _handle(vec_tasks);
            if !re && nofity {
                _handle(vec![]);
            }
            exit || re
        });
    }

    fn stop(&self) {
        self.push_exit();
    }
}

impl<T> ThreadTaskData<T> {
    pub fn push_exit(&self) {
        self.inner.push(TaskData::Exit);
    }
    pub fn push_notify(&self) {
        self.inner.push(TaskData::Notify);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::syncx::{Tasks, ThreadTaskData};

    #[test]
    fn test_thread_task_data() {
        let thread_option = Arc::new(ThreadTaskData::<u32>::with_capacity(10));
        //just for test
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
