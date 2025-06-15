pub mod name;

fn main() {
    let first_name = "John";
    let last_name = "Doe";
    let full_name = name::get_full_name(first_name, last_name); // 调用函数

    println!("Full name: {}", full_name);

    name::age_helper::print_age(25); // 调用子模块函数;
}


