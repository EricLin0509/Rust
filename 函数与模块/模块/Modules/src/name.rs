pub fn get_full_name(first: &str, last: &str) -> String {
    let full_name = format!("{0} {1}", first, last);

    return full_name;
}

pub mod age_helper { // 子模块
    pub fn print_age(age: u8) {
        println!("Age: {}", age);
    }
}