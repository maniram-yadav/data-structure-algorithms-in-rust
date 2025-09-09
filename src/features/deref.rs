use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(data: T) -> Self {
        MyBox(data)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(msg: &str) {
    println!("Hello {msg}");
}

mod test {

    use super::*;
    // #[test]
    fn test_deref() {
        let x = 7;
        let y = MyBox::new(x);
        assert_eq!(x, *y);

        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}
