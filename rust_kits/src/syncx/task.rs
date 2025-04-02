// use std::option::IntoIter;

pub trait Tasks<T> {
    type InterType;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&self, task: T);
    fn pushes(&self, new_tasks: Vec<T>);
    fn take_tasks(&self) -> Vec<Self::InterType>;
    fn swap_tasks(&self, t: &mut Vec<Self::InterType>);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    /// dead loop until the handle return true
    fn sync_run<F: Fn(Vec<T>) -> bool>(&self, handle: F);

    fn stop(&self);
}

pub enum TaskData<T> {
    Exit,
    Data(T),
    Notify,
}

impl<T> From<Option<T>> for TaskData<T> {
    fn from(t: Option<T>) -> Self {
        match t {
            Some(t) => TaskData::Data(t),
            None => TaskData::Exit,
        }
    }
}

impl<T> From<TaskData<T>> for Option<T> {
    fn from(value: TaskData<T>) -> Self {
        match value {
            TaskData::Data(t) => Some(t),
            TaskData::Exit | TaskData::Notify => None,
        }
    }
}

// impl<T> IntoIterator for TaskData<T> {
//     type Item = T;
//     type IntoIter = IntoIter<T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
