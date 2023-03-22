// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
// #![feature(test)]
// #![feature(ptr_metadata)]

#![allow(unused_variables)]
#![allow(unused)]
#![allow(dead_code)]

pub mod caller;
pub mod r_static;
mod ref_pointer;
mod types_;
mod macro_;
mod sized_;
mod format_;
mod enum_;
pub mod doc_;
mod closure_;
mod copy_;
mod env_;
mod async_thread;
mod trait_;
mod lang_;
mod panic_error;
mod proc_;
