fn main() {
    can_drive();
}

fn can_drive() {
    println!("请填写您的年龄：");

    // 获取用户输入的年龄
    let input = &mut String::from("");
    std::io::stdin().read_line(input).unwrap();


    let age = input.replace("\n", "").parse::<u8>().unwrap(); // 转换为 u8 类型

    if age >= 18 { // 条件判断
        println!("您可以考驾照！");
    }
    else {
        println!("您需要年龄 18 岁以上才能考驾照！");
    }

    let drivers_license = if age >= 18 {true} else {false};
    println!("您的驾照是否有效：{}", drivers_license);
}