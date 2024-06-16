pub use cpp_generate::*;

mod cpp_generate;

#[test]
fn test() {
    unsafe {
        {
            let i = 1;
            println!("f_int: {}, p: {:p}", i, &i);
            crate::gen::root::f_int(i);
        }

        {
            let mut i = 2;
            println!("f_int_ref: {}, p: {:p}", i, &i);
            crate::gen::root::f_int_ref(&mut i);
        }
        {
            let mut i = 2;
            println!("f_int_p: {}, p: {:p}", i, &i);
            crate::gen::root::f_int_p(&mut i);
        }
        println!("\n\n");
    }
}
