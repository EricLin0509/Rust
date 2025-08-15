use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T { // 由于 Target 类型是 T，所以可以直接返回 &T
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);

    let m = MyBox::new("Rust");
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}