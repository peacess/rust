
pub trait BytesObj {
    fn bytes_to_obj(bs: &[u8]) -> Self;
}

pub struct Obj{}

impl BytesObj for Obj {
    fn bytes_to_obj(bs: &[u8]) -> Self {
        todo!()
    }
}