# 选项

在 Rust 中，`Option` 类型用于处理可能缺失的值。`Option` 类型是枚举类型，它可以有两个值：`Some(T)` 和 `None`。`Some(T)` 代表有值，`None` 代表缺失值

## 原型

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- `T` 是泛型类型参数
- `Some(T)` 是一个包含值 `T` 的 `Option` 值
- `None` 是一个代表缺失值的 `Option` 值

## 代码示例

### 创建 `Option` 类型

```rust
let x = None; // 缺失值
let y = Some(5); // 有值
let z: Option<i32> = Some(10); // 类型注解
```

### 读取 `Option` 类型的值

使用 `println!` 宏打印 `Option` 类型的值

```rust
let x: Option<i32> = Some(5);
println!("{}" x);
```

此时会报错

```
`Option<i32>` doesn't implement `std::fmt::Display`
the trait `std::fmt::Display` is not implemented for `Option<i32>`
in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

这是因为 `println!` 宏默认使用 `Display` trait 来打印值，而 `Option<T>` 类型没有实现 `Display` trait

所以**要使用 `unwrap()` 方法来获取 `Option` 类型里面的值**

```rust
let x: Option<i32> = Some(5);
println!("{}", x.unwrap());
```

```
5
```

### 尝试读取 `None` 类型的值

```rust
let x: Option<i32> = None;
println!("{}", x.unwrap());
```

运行时会报错

```
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

这是因为 `None` 的值为 `null`，我们不能 `unwarp()` `null` 值

### 修改 `Option` 类型的值

```rust
let mut x: Option<i32> = Some(5);
x = 5;
```

此时会报错

```
mismatched types
expected enum `Option<i32>`
   found type `{integer}`
```

这是因为数据类型不匹配，我们不能将 `i32` 直接赋值给 `Option<i32>` 类型

**要用 `Some()` 来创建 `Option` 类型的值**

```rust
let mut x: Option<i32> = Some(5);
x = Some(10);
```

## 使用自定义类型

我们可以定义自己的类型，作为 `Option` 类型的值

这里使用枚举 (Enum) 来定义 `Option` 类型

```rust
enum Characters {
    Archer,
    Knight,
    Mage,
}
```

假设现在我们想在 `Characters` 枚举类型上添加 `None` 值，来表示没有选择的角色，可以使用 `Option<Characters>` 类型

```rust
fn choose_character() -> Option<Characters> {
    let mut choice = None;
    choice = Some(Characters::Archer);
    return choice;
}
```

### 读取此 `Option` 类型的值

```rust
let character = choose_character();
println!("{}", character.unwrap());
```

此时会报错

```
`Characters` doesn't implement `std::fmt::Display`
the trait `std::fmt::Display` is not implemented for `Characters`
in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

因为 `Characters` 枚举类型不是 `String`、`&str`、`i32` 等基本类型，所以没有实现 `Display` trait

那可以使用 `impl` 块来实现转换成字符串的功能

```rust
impl ToString for Characters {
    fn to_string(&self) -> String {
        match self {
            Characters::Archer => "Archer",
            Characters::Knight => "Knight",
            Characters::Mage => "Mage",
        }.to_string()
    }
}
```

然后就可以使用 `unwrap()` 方法来获取 `Option` 类型的值，并转换成字符串

```rust
println!("{}", character.unwrap().to_string());
```

## 方法

- `is_some()`：判断 `Option` 类型的值是否为 `Some`
- `is_none()`：判断 `Option` 类型的值是否为 `None`
- `unwrap()`：获取 `Some` 类型的值，如果 `Option` 类型的值为 `None`，则会 panic
