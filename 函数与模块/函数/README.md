# 函数

## 简介

在 Rust 中，函数是一种基本的编程构造。函数可以接受参数，执行一些操作，并返回结果

## 语法

```rust
fn 函数名(参数列表) -> 返回类型 {
    // 函数体
}
```

- 参数列表可以为空，也可以包含多个参数，参数之间用逗号分隔
- 返回类型可以为空，也可以指定返回值的类型

## 代码示例

假设我们要编写一个函数，获取姓和名，并返回姓名的全拼

```rust
fn get_full_name(first: &str, last: &str) -> String {    }
```

- `first`: 字符串切片，表示名
- `last`: 字符串切片，表示姓
- `-> String`: 返回值类型为 `String`

### 合并字符串

```rust
fn get_full_name(first: &str, last: &str) -> String {
    let full_name = format!("{0} {1}", first, last);
}
```

- `format!`: 格式化字符串，用 `{}` 包裹变量，并用逗号分隔
- `{0}` / `{1}`: 变量索引，表示第一个参数和第二个参数

### 返回值

使用 `return` 关键字可以返回函数的结果

```rust
fn get_full_name(first: &str, last: &str) -> String {
    let full_name = format!("{0} {1}", first, last);

    return full_name; // 返回值
}
```

此时会有警告

```
function `get_full_name` is never used
```

这是因为函数没有被调用，所以 Rust 编译器会给出警告

可以使用 `#[allow(dead_code)]` 注解来禁止这个警告

```rust
#[allow(dead_code)]
fn get_full_name(first: &str, last: &str) -> String {
    let full_name = format!("{0} {1}", first, last);

    return full_name; // 返回值
}
```

### 调用函数

```rust
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
```

这里需要一个 `full_name` 变量来存储函数的返回值，否则返回值会被丢弃

[源代码](Functions/src/main.rs)