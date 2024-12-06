#![allow(unused_variables)]

#[cfg(test)]
mod test {
    use std::{
        cell::{Cell, OnceCell},
        collections::BTreeMap,
        sync::{Arc, Mutex, Once},
        thread::spawn,
    };

    #[test]
    fn test_sync() {
        let v: &'static Vec<i32> = unsafe { fn_g_vec() };
        println!("len: {}", v.len());
        let t = spawn(move || {
            let cap = v.capacity();
            println!("{}", v.len());
            // v.push(10);
        });
        let v: &'static mut Vec<i32> = unsafe { fn_g_vec() };
        v.push(10);
        let re = t.join();
        {
            let v = vec![0, 1];
            std::thread::spawn(move || {
                let cap = v.capacity();
                println!("{}", v.len());
            });
        }
    }

    #[allow(static_mut_refs)]
    unsafe fn fn_g_vec() -> &'static mut Vec<i32> {
        // if run in multi-thread, use std::sync::OnceLock
        static mut DATA: OnceCell<Vec<i32>> = OnceCell::new();
        DATA.get_or_init(|| vec![0, 1, 2]);
        DATA.get_mut().expect("static is not init")
    }

    #[test]
    fn test_sync_send_has_race() {
        struct Data {
            c: Cell<i32>,
        }
        unsafe impl Sync for Data {}
        unsafe impl Send for Data {}

        impl Data {
            pub fn add(&self) {
                let t = self.c.get() + 1;
                self.c.set(t);
            }
        }

        let d = Arc::new(Data { c: Cell::new(0) });

        let d2 = d.clone();
        let t2 = spawn(move || {
            for i in 0..101 {
                d2.add();
            }
        });
        let d3 = d.clone();
        let t3 = spawn(move || {
            for i in 0..101 {
                d3.add();
            }
        });
        for i in 0..101 {
            d.add();
        }
        let re = t2.join();
        let re = t3.join();
        println!("len: {}", d.c.get());
    }

    #[test]
    fn test_sync_send_no_race() {
        struct Data {
            c: Cell<i32>,
            m: Mutex<i32>,
        }
        unsafe impl Sync for Data {}
        unsafe impl Send for Data {}

        impl Data {
            pub fn add(&self, d: i32) {
                let m = self.m.lock();
                let t = self.c.get() + 1;
                self.c.set(t);
            }
        }

        let d = Arc::new(Data {
            c: Cell::new(0),
            m: Mutex::new(0),
        });

        let d2 = d.clone();
        let t2 = spawn(move || {
            for i in 0..101 {
                d2.add(0);
            }
        });
        let d3 = d.clone();
        let t3 = spawn(move || {
            for i in 0..101 {
                d3.add(0);
            }
        });
        for i in 0..101 {
            d.add(0);
        }
        let re = t2.join();
        if re.is_err() {
            println!("{:?}", re);
        }
        let re = t3.join();
        if re.is_err() {
            println!("{:?}", re);
        }
        println!("len: {}", d.c.take());
    }

    #[test]
    fn test_arc() {
        {
            let mut x = Arc::new(3);
            *Arc::get_mut(&mut x).unwrap() = 4;
            assert_eq!(*x, 4);
            let t = Arc::get_mut(&mut x).unwrap();
            *t = 9;

            let _y = Arc::clone(&x);
            assert!(Arc::get_mut(&mut x).is_none());
            let t = x.clone();
        }
        let mut d = Arc::new(0);

        let md = Arc::get_mut(&mut d).unwrap();
        *md = 4;

        let d2 = d.clone();
        let t1 = spawn(move || {
            println!("{}", d2);
        });
        // *md = 9;
        // t1.join();
    }

    #[test]
    fn test_arc_map() {
        struct Handles {
            maps: BTreeMap<i32, String>,
        }
        let mut arc = Arc::new(Handles { maps: BTreeMap::new() });
        Arc::get_mut(&mut arc).expect("").maps.insert(0, "0".to_owned());

        let mut vec = Vec::new();
        for i in 0..10 {
            let clone = arc.clone();
            let j = std::thread::spawn(move || {
                let v = clone.maps.get(&0);
            });
            vec.push(j);
        }
        for it in vec {
            it.join().expect("");
        }
    }
}
