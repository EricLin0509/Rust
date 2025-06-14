fn main() {
    let a: () = (); // 单值类型

    let b: i8 = 42; // 32位有符号整数
    let c: u32 = 42; // 32位无符号整数

    let d: f32 = 3.14; // 32位浮点数
    let e: f64 = 3.14; // 64位浮点数

    let f: bool = true; // 布尔值
    let g: bool = false; // 布尔值

    let h: char = 'a'; // 字符

    let i: String = "hello world".to_string(); // 字符串
    let j: &str = "hello world"; // 字符串切片
}
