
大量对象需要实现 bytes to object（bytes_to_obj）  
有如下几种实现：  
1.  给每一个对象实现一个方法 bytes_to_obj  
2.  定义一个trait BytesObj，给ByteObj一个范型实现  
3.  实现一个范型方法  

```rust
pub trait BytesObj {
    fn bytes_to_obj(bs: &[u8]) -> Self;
}
pub trait ObjTrait{}
// m1
pub struct Obj{}
impl BytesObj for Obj {
    fn bytes_to_obj(bs: &[u8]) -> Self {
        todo!()
    }
}

// m2
impl<T: ObjTrait> BytesObj for T {
    fn bytes_to_obj(bs: &[u8]) -> Self {
        todo!()
    }
}

// m3
pub fn bytes_to_obj<T: ObjTrait>(bs: &[u8]) -> T {
    todo!()
}
```
最后我选择了m3的实现，理由： 
m1 是通常的实现方法，也是第一个会想到的方法，Obj与实现是强关系，且一定会有  
  （注：这个实现可以使用macro等方式，让代码在一处），  
   很方便为不同类型做不同实现，但不管是否需要都会有代码，且代码在不同地方  
m2是给范型实现trait，代码只有一处，与Obj的关系是，当使用时有且一定会有，在理解上需要多转一个弯（给范型实现trait）
m3是范型函数，当有使用时才会生成代码，与Obj的关系是，当使用时有且一定会有。  
   它比m2的优点在于非常好理解，因为现在rust还不支持“特化”，所以不方便给不同类型做不同实现
