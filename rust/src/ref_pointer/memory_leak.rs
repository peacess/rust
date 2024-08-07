#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;
use sysinfo::System;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const FOR_1: i32 = 1_00;
const FOR_2: i32 = 1_000;
const FOR_LEN: i32 = 3;
// 默认内存分配输出，
// 从中可以看出，vec与binary heap运行后，内存全部释放
// btree map与hashmap都没有安全释放
//              Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// vec:         Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// vec:         Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// vec:         Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// binary heap: Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// binary heap: Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// binary heap: Statm { size: 858, resident: 480, share: 480, text: 99, data: 93 }
// btree map:   Statm { size: 875, resident: 528, share: 480, text: 99, data: 110 }
// btree map:   Statm { size: 875, resident: 528, share: 480, text: 99, data: 110 }
// btree map:   Statm { size: 875, resident: 528, share: 480, text: 99, data: 110 }
// hash map:    Statm { size: 888, resident: 528, share: 480, text: 99, data: 123 }
// hash map:    Statm { size: 888, resident: 528, share: 480, text: 99, data: 123 }
// hash map:    Statm { size: 888, resident: 528, share: 480, text: 99, data: 123 }

// Jemalloc内存分配输出，
// 从中可以看出，vec、binary heap、btree map、hash map 运行后，内存全部释放
// btree map与hashmap使用的内存更多
//              Statm { size: 4068, resident: 816, share: 768, text: 694, data: 2669 }
// vec:         Statm { size: 4068, resident: 960, share: 864, text: 694, data: 2669 }
// vec:         Statm { size: 4068, resident: 960, share: 864, text: 694, data: 2669 }
// vec:         Statm { size: 4068, resident: 960, share: 864, text: 694, data: 2669 }
// binary heap: Statm { size: 4068, resident: 960, share: 864, text: 694, data: 2669 }
// binary heap: Statm { size: 4068, resident: 960, share: 864, text: 694, data: 2669 }
// binary heap: Statm { size: 4068, resident: 960, share: 864, text: 694, data: 2669 }
// btree map:   Statm { size: 4068, resident: 1008, share: 864, text: 694, data: 2669 }
// btree map:   Statm { size: 4068, resident: 1008, share: 864, text: 694, data: 2669 }
// btree map:   Statm { size: 4068, resident: 1008, share: 864, text: 694, data: 2669 }
// hash map:    Statm { size: 4068, resident: 1056, share: 864, text: 694, data: 2669 }
// hash map:    Statm { size: 4068, resident: 1056, share: 864, text: 694, data: 2669 }
// hash map:    Statm { size: 4068, resident: 1056, share: 864, text: 694, data: 2669 }
fn main() {
    let mut sys = sysinfo::System::new_all();
    if let Some(p) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!("             {:?}", p.memory());
    }
    for _ in 0..FOR_LEN {
        vector(&mut sys);
    }
    for _ in 0..FOR_LEN {
        binary_heap(&mut sys);
    }
    for _ in 0..FOR_LEN {
        btree_map(&mut sys);
    }
    for _ in 0..FOR_LEN {
        hash_map(&mut sys);
    }
}

fn vector(sys: &mut System) {
    for i in 0..FOR_1 {
        let mut data = Vec::new();
        for j in 0..FOR_2 {
            let s = format!("{}_{}", i, j);
            data.push(s);
        }
        data.clear();
    }
    sys.refresh_all();
    if let Some(p) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!("vec:         {:?}", p.memory());
    }
}

fn binary_heap(sys: &mut System) {
    for i in 0..FOR_1 {
        let mut data = std::collections::BinaryHeap::new();
        for j in 0..FOR_2 {
            let s = format!("{}_{}", i, j);
            data.push(s);
        }
        data.clear();
    }
    sys.refresh_all();
    if let Some(p) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!("binary heap: {:?}", p.memory());
    }
}

fn btree_map(sys: &mut System) {
    for i in 0..FOR_1 {
        let mut data = std::collections::BTreeMap::new();
        for j in 0..FOR_2 {
            let s = format!("{}_{}", i, j);
            data.insert(s.clone(), s);
        }
        data.clear();
    }
    sys.refresh_all();
    if let Some(p) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!("btree map:   {:?}", p.memory());
    }
}

fn hash_map(sys: &mut System) {
    for i in 0..FOR_1 {
        let mut data = std::collections::HashMap::new();
        for j in 0..FOR_2 {
            let s = format!("{}_{}", i, j);
            data.insert(s.clone(), s);
        }
        data.clear();
    }
    sys.refresh_all();
    if let Some(p) = sys.process(sysinfo::Pid::from(std::process::id() as usize)) {
        println!("hash map:    {:?}", p.memory());
    }
}
