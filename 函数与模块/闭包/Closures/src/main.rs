fn main() {
    let greeting = || println!("Hello, world!");
    greeting();

    let print_sum = |a, b| println!("{} + {} = {}", a, b, a + b);
    print_sum(2, 3);

    let x = 10;
    let add = |y| x + y;
    println!("{} + 5 = {}", x, add(5)); // 10 + 5 = 15

    let mut y = 20;
    let mut change_y = || y = 30;
    change_y();
    println!("y = {}", y); // 30
}
