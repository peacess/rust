use std::ffi::c_char;

mod gen;

fn main() {
    unsafe {
        {
            let i = 1;
            println!("f_int: {}, p: {:p}", i, &i);
            gen::root::f_int(i);
        }

        {
            let mut i = 2;
            println!("f_int_ref: {}, p: {:p}", i, &i);
            gen::root::f_int_ref(&mut i);
        }
        {
            let mut i = 2;
            println!("f_int_p: {}, p: {:p}", i, &i);
            gen::root::f_int_p(&mut i);
        }
        println!("\n\n");
    }
    unsafe {
        {
            let i: c_char = 40;
            println!("f_char: {}, p: {:p}", i, &i);
            gen::root::f_char(i);
        }

        {
            let mut i: c_char = 50;
            println!("f_char_ref: {}, p: {:p}", i, &i);
            gen::root::f_char_ref(&mut i);
        }
        {
            let mut i: c_char = 60;
            println!("f_char_p: {}, p: {:p}", i, &i);
            gen::root::f_char_p(&mut i);
        }
        println!("\n\n");
    }
    unsafe {
        {
            let data = gen::root::Data { a: 1, d: 40 as c_char };
            println!("f_data: ,p: {:p}", &data);
            gen::root::f_data(data);
            println!();
        }

        {
            let mut data: gen::root::Data = gen::root::Data { a: 2, d: 50 as c_char };
            println!("f_data_ref: ,p: {:p}", &data);
            gen::root::f_data_ref(&mut data);
            println!();
        }
        {
            let mut data = gen::root::Data { a: 3, d: 60 as c_char };
            println!("f_data_p: ,p: {:p}", &data);
            gen::root::f_data_p(&mut data);
            println!();
        }
        println!("\n\n");
    }
}
