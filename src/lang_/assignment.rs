#[test]
fn test_assign() {
    {
        let t: usize = 0;
        let mut t = 1;
        let ref1_t = &t;//ref_t1为 &T类型
        // ref1_t = &t1; //不是mut &，不能二次赋值
        // *ref1_t = 2;  //不是 mut t，不能二次赋值

        let ref2_t = &mut t;
        // ref2_t = &t2;  //不是mut &，不能二次赋值
        *ref2_t = 3;//是 mut t，能二次赋值

        let mut ref_t3 = &mut t;
        ref_t3 = &mut t; //是mut &，能二次赋值
        *ref_t3 = 2;  //是 mut t，能二次赋值
    }
    {//在let赋值时，栈部分是内存copy的，也就是一个浅copy
        /// 总结
        /// 1. 非引用的赋值时会产生一个新的对象，并作一次浅copy -- 这是所有权转移。传参数也是一种赋值
        /// 2. let ref a = ..与 let a = &..产生的变量是一样的，ref多用在match中
        /// 3. 引用是一种指针，引用本身需要内存来存放，实质上是个usize类型
        /// 4. 编译器会确保引用是有效的，而raw指针要自己管理，所以引用是一种安全的指针
        /// 5. c++中的引用是一个别名，它们是同一个对象，没有新的内存分配

        let v = vec![0];
        println!("&v:       {:p}", &v);
        let v2 = v;
        println!("&v2:      {:p}", &v2);
        let ref ref_v2 = v2;
        println!("ref_v2:   {:p}", ref_v2);
        println!("&ref_v2:  {:p}", &ref_v2);
        let ref ref2_v2 = v2;
        println!("ref2_v2:  {:p}", ref2_v2);
        println!("&ref2_v2: {:p}", &ref2_v2);

        let ref3_v2 = &v2;
        println!("ref3_v2:  {:p}", ref3_v2);
        println!("&ref3_v2: {:p}", &ref3_v2);
        let ref4_v2 = &v2;
        println!("ref4_v2:  {:p}", ref4_v2);
        println!("&ref4_v2: {:p}", &ref4_v2);

        //其中一次输出
        // &v:       0xc63afef38
        // &v2:      0xc63afefa0
        // ref_v2:   0xc63afefa0
        // &ref_v2:  0xc63aff010
        // ref2_v2:  0xc63afefa0
        // &ref2_v2: 0xc63aff0b0
        // ref3_v2:  0xc63afefa0
        // &ref3_v2: 0xc63aff150
        // ref4_v2:  0xc63afefa0
        // &ref4_v2: 0xc63aff1f0
    }
}