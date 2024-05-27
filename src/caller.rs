#[cfg(test)]
mod test {
    //see https://stackoverflow.com/questions/63164973/why-does-rust-allow-calling-functions-via-null-pointers
    fn foo() {
        println!("This is really weird...");
    }

    fn caller<F>()
    where
        F: FnMut(),
    {
        let closure_ptr = 0 as *mut F;
        let closure = unsafe { &mut *closure_ptr };
        closure();
    }

    fn create<F>(_: F)
    where
        F: FnMut(),
    {
        let f = caller::<F>;
        f();
    }

    #[test]
    fn test_run() {
        create(foo); //参数是一种类型， 对于 foo这种类型来说，不管这种类型的值是什么，都可以通过它来调用函数

        create(|| println!("Okay...")); //参数是一种类型

        let val = 42;
        create(|| println!("This will seg fault: {}", val)); //参数包含了数据，所以直接通过null值来调用是有问题的。
    }
}
