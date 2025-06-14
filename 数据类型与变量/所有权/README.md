# 所有权

## 简介

在 Rust 中，所有权 (Ownership) 是 Rust 最独特的特征。它定义了值 (Value) 在内存中的位置以及值可以被哪些代码访问

所有权与变量的作用域密切相关。当变量进入作用域时，它绑定到一个值。当变量离开作用域时，它绑定的值将被丢弃

## 所有权规则

- 每个值都有一个所有者 (Owner)
- 每个值只能有一个所有者
- 当所有者超出作用域时，值将被丢弃


## 代码示例

### 每个值都有一个所有者

```rust
fn main() {
    let string = String::from("hello");
    let len = calculate(&string);
    println!("The length of {} is {}.", string, len);
}

fn calculate(s: &String) -> usize {
    s.len()
}
```

在此示例中，`calculate` 函数接收一个指向 `String` 值的引用 `s`。`s` 并没有改变 `String` 值的所有者，所以在 `calculate` 函数返回后，`str` 仍然是有效的

### 每个值只能有一个所有者

```rust
fn main() {
    let str1 = String::from("hello");
    let str2 = str1;
    println!("{}", str2);

    println!("{}", str1); // 错误，str1的所有者已经移动到str2中，不能再使用
}
```

在此示例中，当 `str2` 使用 `str1` 这个变量赋值时，`str1` 的值被移动到了 `str2` 中。这意味着 `str1` 不再有效，所以不能再使用它。

### 当所有者超出作用域时，值将被丢弃

```rust
fn main() {
    let string = String::from("hello");
    println!("{}", string);
}

fn print() {
    println!("{}", string); // 错误，已经超出了string的作用域，不能再使用
}
```

在 `print` 函数中，尝试使用 `string` 变量，但是 `string` 已经超出了作用域，所以会报错