
pub trait BytesObj {
    fn bytes_to_obj(bs: &[u8]) -> Self;
}

pub trait ObjTrait{}

impl<T: ObjTrait> BytesObj for T {
    fn bytes_to_obj(bs: &[u8]) -> Self {
        todo!()
    }
}