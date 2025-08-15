# Deref特征

Deref特征允许我们自定义对智能指针的解引用行为

## 代码示例

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);
}
```

这个可以正常运行，因为我们对 `y` 的解引用操作是直接对 `x` 的引用

现在我们使用 `Box` 来存储指针

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, *y);
}
```

这个也能正常运行，因为 `Box` 实现了 `Deref` 特征，所以我们可以对 `Box` 的指针进行解引用操作，得到 `Box` 内部存储的值

## 创建自定义的智能指针

现在我们要创建一个自定义的智能指针 `MyBox`

```rust
struct MyBox<T>(T);
```

这个结构体有一个泛型参数 `T`，表示内部存储的值的类型

```rust
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

### 调用 `MyBox`

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);
}
```

此时会报错

```
type `MyBox<{integer}>` cannot be dereferenced
can't be dereferenced
```

这是因为 `MyBox` 没有实现 `Deref` 特征，所以我们不能对 `MyBox` 的指针进行解引用操作

### 实现 `Deref` 特征

#### 调用 `Deref` 特征

```rust
use std::ops::Deref;
```

#### 实现 `Deref` 特征

```rust
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

- `type Target = T;` 定义了 `Deref` 特征的返回类型
- `fn deref(&self) -> &Self::Target` 定义了 `Deref` 特征的解引用行为，返回内部存储的值的引用

所以我们现在解引用的时候，会调用 `deref` 方法

```rust
assert_eq!(5, *(y.deref()));
```

但是 Rust 会自动帮我们调用 `deref` 方法，所以我们可以直接写成 `*y`

### 自动解引用类型转换

自动解引用类型转换 (Automatic Deref Coercion) 是 Rust 编译器的一个特性，它允许我们对智能指针进行类型转换，而不需要手动调用 `deref` 方法

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new("Rust");
    hello(&m);
}
```

当把 `m` 传递给 `hello` 函数时，Rust 编译器会自动对 `m` 进行解引用

第一次解引用 `m` 时，会得到 `&String` 类型

然后再次解引用 `&String` 类型得到 `&str` 类型，最后调用 `hello` 函数

```
&MyBox<String> -> &String -> &str
```

#### 规则

- 不可变引用转换为不可变引用：`&T` -> `&U`
- 可变引用转换为可变引用：`&mut T` -> `&mut U`
- 可变引用转换为不可变引用：`&mut T` -> `&U`
- **但不能将不可变引用转换为可变引用**