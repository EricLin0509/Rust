fn main() {
    let first_name = "John";
    let last_name = "Doe";
    let full_name = get_full_name(first_name, last_name); // 调用函数

    println!("Full name: {}", full_name);
}

fn get_full_name(first: &str, last: &str) -> String {
    let full_name = format!("{0} {1}", first, last);

    return full_name;
}