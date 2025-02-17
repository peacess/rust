// #![feature(unboxed_closures)]// #![feature(unboxed_closures)]

// #![feature(fn_traits)]
// #![feature(test)]
// #![feature(ptr_metadata)]

#![allow(unused_variables)]
#![allow(unused)]
#![allow(dead_code)]

mod async_thread;
pub mod caller;
mod closure_;
mod copy_trait;
mod design;
pub mod doc_;
mod env_;
mod format_;
mod lang_;
mod life_;
mod macro_;
mod net_;
mod panic_error;
mod ref_pointer;
mod trait_;

mod args;
mod generics_;
