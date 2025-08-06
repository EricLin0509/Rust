fn main() {
    let x = Box::new(5);
    println!("x: {}", *x);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}