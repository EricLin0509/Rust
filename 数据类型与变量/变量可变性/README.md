# 变量可变性

## 简介

在 Rust 中，变量默认是不可变的 (immutable) 。这意味着一旦一个变量被绑定到一个值，就不能改变这个变量的值

## 可变变量

假设现在有一个变量 `x`，然后修改它的值

```rust
let x = 5;
x = 10;
```

此时会报错

```
cannot assign twice to immutable variable `x`
```

这是因为 `x` 是不可变的，不能被修改两次。

### 声明可变变量

要声明一个可变变量，需要在变量名前添加 `mut` 关键字。

```rust
let mut x = 5;
```

### 修改变量值

可以修改变量绑定的值，因为变量是可变的。

```rust
let mut x = 5;
x = 10;
println!("x = {}", x);
```
