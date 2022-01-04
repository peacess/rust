
#[test]
fn test_assign() {
    {//在let赋值时，栈部分是内存copy的，也就是一个浅copy
        let v = vec![0];
        println!("&v:       {:p}", &v);
        let v2 = v;
        println!("&v2:      {:p}", &v2);
        let ref ref_v2 = v2;
        println!("ref_v2:   {:p}",ref_v2);
        println!("&ref_v2:  {:p}",&ref_v2);
        let ref ref2_v2 = v2;
        println!("ref2_v2:  {:p}",ref2_v2);
        println!("&ref2_v2: {:p}",&ref2_v2);

        let ref3_v2 = &v2;
        println!("ref3_v2:  {:p}",ref3_v2);
        println!("&ref3_v2: {:p}",&ref3_v2);
        let ref4_v2 = &v2;
        println!("ref4_v2:  {:p}",ref4_v2);
        println!("&ref4_v2: {:p}",&ref4_v2);

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
        /// 总结
        /// 1. 使用let赋值时会产生一次栈的copy，也就是一次浅copy
        /// 2. let ref a = ..与 let a = &..产生的变量是一样的，let ref少用在定义变量，多用在match中
        /// 3. 引用是一种指针，引用本身需要内存来存放
        /// 4. 定义一个引用就会分配内存来存放引用本身，引用指向同一个对象
        /// 5. 与raw指针不一样的是，编译器会确保引用是有效的，而raw指针自己来管理
        /// 6. c++中的引用是一个别名，它们是同一个对象，没有新的内存分配
    }
}