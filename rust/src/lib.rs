// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
// #![feature(test)]
// #![feature(ptr_metadata)]

#![allow(unused_variables)]
#![allow(unused)]
#![allow(dead_code)]

mod async_thread;
pub mod caller;
mod closure_;
mod copy_;
mod design;
pub mod doc_;
mod env_;
mod format_;
mod lang_;
mod life_;
mod net_;
mod panic_error;
mod proc_;
mod ref_pointer;
mod trait_;

mod args;
mod macro_;
