fn main() {
    let string = String::from("hello");
    let len = calculate(&string);
    println!("The length of {} is {}.", string, len);
}

fn calculate(s: &String) -> usize {
    s.len()
}