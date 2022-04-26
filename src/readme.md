# async_thread

## block_on_

## [order_](./async_thread/order_.rs)

1. ordering只是一个编译标识，编译器会使用不低于它的级别。
2. 它与硬件或cpu架构都有关，写代码时给出最低要求的级别就可以，运行时的级别，很难确定，也很验证
3.

[std::sync::atomic::Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)  
[c++ memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order)  
[loop,测试Ordering](https://github.com/tokio-rs/loom)  
[Atomics and Memory Ordering](https://riptutorial.com/rust/example/21259/atomics-and-memory-ordering)  
[C++11 atomic x86 memory ordering](https://stackoverflow.com/questions/11836028/c11-atomic-x86-memory-ordering)  
[Rust 并发编程 - Memory Ordering](https://www.jianshu.com/p/511cde6b62a6)

# copy_

在Rust中Copy trait没有方法，

* copy动着是由编译器生成按位复制的，
* 虽然Copy一定要求Clone，但在copy发生时，并不会调用Clone中的方法（这是我以前理解错误的地方）
* 有Copy，那么要求clone方法与copy产生相同的效果（这是一个约束，并没有编译器要求），也就是这样实现： fn clone(&self) -> Self{
  *self } 所以大部都是#[derive(Copy, Clone)]，如果是其它形式，给出足够足够的理由