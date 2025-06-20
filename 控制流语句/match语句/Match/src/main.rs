fn main() {
    let age = 20;
    let has_driving_license = true;

    match age {
        0..18 => println!("你还未成年"),
        18..=150 if has_driving_license => println!("你已经成年且考了驾照"), // 添加条件表达式
        18..=150 if !has_driving_license => println!("你已经成年但未考驾照"), // 添加条件表达式
        _ => println!("你输入的年龄无效"),
    }

    let mut region = get_manfacturer_region("Toyota");
    println!("{}的产地是：{}", "Toyota", region);
    region = get_manfacturer_region("BMW");
    println!("{}的产地是：{}", "BMW", region);

    match_array();
}

fn get_manfacturer_region(manufacturer: &str) -> &str {
    match manufacturer {
        "Toyota" => "Japan",
        "Honda" => "Japan",
        "Ford" => "USA",
        "BMW" => "Germany",
        "Mercedes" => "Germany",
        "Porsche" => "Germany",
        "Audi" => "Germany",
        _ => "Unknown",
    }
}

fn match_array() {
    let price: [u32; 4] = [100, 200, 300, 400];
    match price[0..=1] { // 判断数组的前两个元素
        [100, 200] => println!("价格区间为100-200"),
        [200, 300] => println!("价格区间为200-300"),
        _ => println!("数组元素超出范围"),
    }
    match price[0..=3] {
        [100, ..] => println!("价格区间为100-400"),
        [200, ..] => println!("价格区间为200-400"),
        [300, ..] => println!("价格区间为300-400"),
        _ => println!("数组元素超出范围"),
    }
}