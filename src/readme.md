
# copy_
在Rust中Copy trait没有方法，
* copy动着是由编译器生成按位复制的，
* 虽然Copy一定要求Clone，但在copy发生时，并不会调用Clone中的方法（这是我以前理解错误的地方）
* 有Copy，那么要求clone方法与copy产生相同的效果（这是一个约束，并没有编译器要求），也就是这样实现：
fn clone(&self) -> Self{
    *self
}
所以大部都是#[derive(Copy, Clone)]，如果是其它形式，给出足够足够的理由