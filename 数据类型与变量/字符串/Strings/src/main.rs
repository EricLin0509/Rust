use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = String::new(); // 空字符串
    let s2 = "Hello world"; // 字符串切片
    let s3 = s2.to_string();
    let s4 = String::from("Hello world"); // 通过 String::from() 方法创建字符串

    let mut string = String::from("Hello ");
    println!("{}", string);
    string.push('w');
    string.push_str("orld");
    println!("{}", string);

    for c in "你好，世界！".chars() {
        println!("{}", c);
    }

    for b in "你好，世界！".bytes() {
        println!("{}", b);
    }

    for g in "你好，世界！".graphemes(true) {
        println!("{}", g);
    }
}
