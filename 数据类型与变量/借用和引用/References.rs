fn main() {
    let mut a = 10;
    let b = &mut a;
    *b += 10;

    println!("a: {}", a); // 借用 a 的值
}