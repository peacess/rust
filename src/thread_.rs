use std::cell::Cell;
use std::sync::Once;
use std::thread::spawn;

#[test]
fn test_sync() {
    let v: &'static Vec<i32> = FN_G_VEC();
    println!("len: {}", v.len());
    let t = spawn(move || {
        let cap = v.capacity();
        println!("{}", v.len());
        // v.push(10);
    });
    let mut v: &'static mut Vec<i32> = FN_G_VEC();
    v.push(10);
    t.join();
    {
        let v = vec![0, 1];
        std::thread::spawn(move || {
            let cap = v.capacity();
            println!("{}", v.len());
        });
    }
}

fn FN_G_VEC() -> &'static mut Vec<i32> {
    static one: Once = Once::new();
    static mut data: Cell<Option<Vec<i32>>> = Cell::new(None);
    one.call_once(|| {
        unsafe { data.set(Some(vec![0, 1, 2])); }
    });
    unsafe { data.get_mut() }.as_mut().expect("static is not init")
}