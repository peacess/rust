#![allow(unused_variables)]

#[allow(dead_code)]
#[test]
fn test_enum() {
    pub enum ScalarValue {
        Bytes(Vec<u8>),
        Str(String),
        Int(i64),
        Uint(u64),
        F64(f64),
        Counter(i64),
        Timestamp(i64),
        Boolean(bool),
    }

    let t = vec![ScalarValue::Boolean(false)];
}