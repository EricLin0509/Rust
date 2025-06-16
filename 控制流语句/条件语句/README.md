# 条件语句

## 简介

在 Rust 中，条件语句有 `if-else`、`if let`、`match` 三种

- `if-else`：最基本的条件语句，根据条件判断是否执行代码块
- `if let`：用于模式匹配，根据模式匹配是否成功执行代码块
- `match`：用于多分支条件判断，根据表达式的值执行不同的代码块

## 代码示例

### `if-else`

```rust
if 条件表达式 {
    // 条件成立时执行的代码块
}
else { // else 语句是可选的
    // 条件不成立时执行的代码块
}
```

假设现在要判断某人的年龄是否可以考驾照 (大于等于 18 岁)，可以用 `if-else` 语句实现

```rust
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
}
```

- `read_line()`: 从标准输入读取一行输入，并将其存储在 `input` 变量中
- `unwrap()`: 成功读取输入时，返回 `Ok`，失败时 panic
- `replace()`: 删除 `input` 末尾的换行符 `\n`
- `parse()`: 尝试将 `input` 转换为 `u8` 类型，失败时 panic

```
cargo run
请填写您的年龄：
20
您可以考驾照！

cargo run
请填写您的年龄：
16
您需要年龄 18 岁以上才能考驾照！
```

可以看到，根据用户输入的年龄，程序输出不同的信息

### 使用 `if-else` 语句赋值

```rust
let 变量名 = if 条件表达式 {
    // 条件成立时的值
}
else { // else 语句是可选的
    // 条件不成立时的值
}
```

使用 `if-else` 语句赋值，可以根据条件表达式的成立与否，给变量赋值不同的值

```rust
let drivers_license = if age >= 18 {true} else {false};
```