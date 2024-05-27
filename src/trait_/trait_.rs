trait A {
    fn f(&mut self);
}

trait B {
    fn fb(&mut self);
}

struct Data {}

impl Data {
    fn d(&mut self) {
        println!("data::d");
    }
}

impl A for Data {
    fn f(&mut self) {
        self.d();
    }
}

impl B for Data {
    fn fb(&mut self) {
        println!("B::fb");
        self.f();
    }
}

#[test]
fn test_data() {
    let mut d = Data {};
    d.f();
    d.fb();
}
