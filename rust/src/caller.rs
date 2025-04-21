#[cfg(test)]
mod test {
    use std::{env, process::Command};

    use regex::Regex;

    //see https://stackoverflow.com/questions/63164973/why-does-rust-allow-calling-functions-via-null-pointers
    // the channel = "1.80.1" can work, but not work in the 1.86.0
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
        let mut version = "".to_string();
        {
            let mut output = Command::new("cargo").arg("--version").output().unwrap();
            let full_version = String::from_utf8(output.stdout).unwrap();
            let reg = Regex::new(r".*(\d+\.\d+\.\d*) .*").unwrap();
            if let Some(cap) = reg.captures(&full_version) {
                if let Some(da) = cap.get(1) {
                    version = da.as_str().to_string();
                }
            }
        }
        if version.as_str() <= "1.80.1" {
            create(foo); //参数是一种类型， 对于 foo这种类型来说，不管这种类型的值是什么，都可以通过它来调用函数
            create(|| println!("Okay...")); //参数是一种类型
        }

        // let val = 42;
        // create(|| println!("This will seg fault: {}", val)); //参数包含了数据，所以直接通过null值来调用是有问题的。
    }
}
