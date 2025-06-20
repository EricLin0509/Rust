# Match 语句

## 简介

match 语句是 Rust 中的一个控制流语句，它可以用来匹配表达式的值，并执行相应的分支代码

## 语法

```rust
match 变量名 {
    模式1 => 表达式1,
    模式2 => 表达式2,
    模式3 => { 代码块; },
    ... // 可以有多个模式和表达式
    _ => 表达式n, // 通配符模式，匹配所有其他情况
}
```

- `变量名`：需要匹配的值
- `模式`：匹配的值的模式，可以是具体的值，也可以是变量名、通配符、范围、元组、结构体、数组等
- `表达式`：当模式匹配成功时执行的表达式

- **注意: match 语句<mark>必须穷尽所有可能的模式</mark>，否则编译器会报错**

## 代码示例

判断是否成年

```rust
fn main() {
    let age = 20;
    match age {
        0..18 => println!("你还未成年"),
        18..=150 => println!("你已经成年了"),
        _ => println!("你输入的年龄无效"),
    }
}
```

## 模式语法

模式有以下几种

- 范围模式
   - 半开半闭区间：`1..100`
   - 闭区间：`1..=100`
   - 范围从: `1..`
   - 范围到开区间：`..100`
   - 范围到闭区间：`..=100`
   - 过时的范围模式：`1...100` (已弃用)

- 值模式
   - 字面值：`10`
   - 变量名：`x`
   - 元组：`(1, 2)`
   - 组合模式：`0 | 1`
   - 结构体：`Point { x: 1, y: 2 }`
   - 数组：`[1, 2, 3]`
   - 通配符模式: `_`

## Arm Guard

match 语句的分支可以添加一个条件表达式，只有满足条件的分支才会被执行

假设现在要同时判断是否成年且考了驾照

```rust
fn main() {
    let age = 20;
    let has_driving_license = true;

    match age {
        0..18 => println!("你还未成年"),
        18..=150 if has_driving_license => println!("你已经成年且考了驾照"), // 添加条件表达式
        18..=150 if !has_driving_license => println!("你已经成年但未考驾照"), // 添加条件表达式
        _ => println!("你输入的年龄无效"),
    }
}
```

## 使用不同的数据类型

```rust
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
```