# 测试

在 Rust 中，我们可以使用 `cargo` 工具来管理项目的测试

## 代码示例

假设现在有一个函数 `add` 用来计算两个数字的和

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

我们可以编写如下测试代码

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(10, 0), 10);
    assert_eq!(add(-1, -2), -3);
}
```

- `#[test]` 注解用来标记一个函数为测试函数，`cargo` 会自动运行这些函数并检查其结果

- `aasert_eq!` 宏用来判断两个值是否相等，如果不相等，会触发一个测试失败的异常

运行 `cargo test` 命令，可以看到测试结果：

```
running 1 test
test test_add... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 测试失败

```rust
#[test]
fn failing() {
    panic!("This test should fail");
}
```

运行 `cargo test` 命令，可以看到测试结果：

```
running 2 tests
test test_add ... ok
test failing ... FAILED

failures:

---- failing stdout ----

thread 'failing' panicked at src/main.rs:18:5:
This test should fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    failing

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

可以看到，测试失败了，并且显示了失败的详细信息

### 实战示例——判断是否能组成三角形

我们现在有一个 `is_triangle` 函数，用来判断三个边长是否能组成一个三角形

```rust
fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && b + c > a && c + a > b
}
```

我们可以编写如下测试代码：

```rust
#[test]
fn is_vaild_triangle() {
    assert_eq!(is_triangle(3, 4, 5), true);
    assert_eq!(is_triangle(5, 12, 13), true);
    assert_eq!(is_triangle(1, 2, 3), false);
}
```

运行 `cargo test` 命令，可以看到测试结果

```
running 1 test
test is_vaild_triangle ... ok

successes:

successes:
    is_vaild_triangle

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 测试模块

一般情况下，我们会把所有测试代码封装在一个独立的模块中，这样可以更方便地管理和运行测试

```rust
// lib.rs

pub fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && b + c > a && c + a > b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vaild_triangle() {
        assert_eq!(is_triangle(3, 4, 5), true);
        assert_eq!(is_triangle(5, 12, 13), true);
        assert_eq!(is_triangle(1, 2, 3), false);
    }
}
```

- `#[cfg(test)]` 用来控制测试模块是否被编译，只有在测试模式下才会编译测试模块

```
running 1 test
test lib::tests::is_vaild_triangle ... ok

successes:

successes:
    lib::tests::is_vaild_triangle

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### assert 宏

有三种 assert 宏：

- `assert!(expression)`：如果**表达式为 `false`**，则触发一个断言失败的异常

```rust
assert!(1 + 1 == 2); // 不发生异常
assert!(1 + 1 == 3); // 发生异常
```

- `assert_eq!(left, right)`：判断两个值**是否相等**，如果不相等，则触发一个断言失败的异常

```rust
assert_eq!(1 + 1, 2); // 不发生异常
assert_eq!(1 + 1, 3); // 发生异常
```

- `assert_ne!(left, right)`：判断两个值**是否不相等**，如果相等，则触发一个断言失败的异常

```rust
assert_ne!(1 + 1, 3); // 不发生异常
assert_ne!(1 + 1, 2); // 发生异常
```

### 自定义断言信息

我们只需要在 `assert` 宏后面添加一个字符串参数，即可自定义断言失败时的信息

```rust
assert_eq!(1 + 1, 3, "1 + 1 should equal 2"); // 发生异常，显示自定义信息
```

```
assertion `left == right` failed: 1 + 1 should equal 2
```

### 测试函数是否会 panic

使用 `#[should_panic]` 注解可以标记一个测试函数，用来测试函数是否会 panic

假设我们现在有一个函数 `divide` 用来计算两个数字的商，如果第二个参数为 0，则会 panic

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Should not divide by zero");
    }
    return a / b;
}
```

如果我们想测试除以 0 的情况，可以使用 `#[should_panic]` 注解

```rust
#[test]
#[should_panic]
fn divide_by_zero() {
    divide(1, 0);
}
```

运行 `cargo test` 命令，可以看到测试结果

```
running 1 test
test lib::test_divide::divide_by_zero - should panic ... ok

successes:

---- lib::test_divide::divide_by_zero stdout ----

thread 'lib::test_divide::divide_by_zero' panicked at src/lib.rs:19:9:
Should not divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: Test::lib::divide
             at ./src/lib.rs:19:9
   3: Test::lib::test_divide::divide_by_zero
             at ./src/lib.rs:30:9
   4: Test::lib::test_divide::divide_by_zero::{{closure}}
             at ./src/lib.rs:29:24
   5: core::ops::function::FnOnce::call_once
             at /usr/src/debug/rust/rustc-1.88.0-src/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


successes:
    lib::test_divide::divide_by_zero

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

可以看到，测试成功，并且显示了 panic 的详细信息

如果测试函数没有 panic，则会显示如下信息：

```
note: test did not panic as expected
```

### 忽略测试

有时，我们只想运行部分测试，忽略其他测试。可以使用 `ignore` 注解来忽略测试

```rust
#[test]
#[ignore] // 忽略测试
fn ignored_test() {
    assert_eq!(1 + 1, 3);
}
```

如果运行 `cargo test` 命令，则不会运行 `ignored_test` 测试

如果想运行所有测试，包括忽略的测试，可以使用 `--ignored` 参数

```
cargo test -- --ignored
```

### 集成测试

集成测试是指测试多个模块或多个函数的集成情况

我们要在 cargo 项目根目录下创建一个 `tests` 文件夹，并在其中创建一个 `.rs` 文件，用来编写集成测试

一般情况下，cargo 会自动检测将根目录变成一个 crate，并编译成可执行文件

```rust
// tests/integration_test.rs

use Test::add; // 导入要测试的模块

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(10, 0), 10);
    assert_eq!(add(-1, -2), -3);
}
```

- **注意：集成测试只能在测试公开接口时使用，不能测试私有接口**


#### 集成测试模块

例如现在我们要在集成测试中添加一个 `common` 模块

如果我们直接在 `tests` 文件夹下创建 `common.rs` 文件，会导致 cargo **将它识别成一个集成测试**

```rust
// tests/common.rs

pub fn setup() {
    println!("setup");
}
```

```
Running tests/common.rs (target/debug/deps/common-b9b7236f69a53017)
```

如果我们想让它成为一个独立的模块，需要创建一个 `common` 文件夹，并在其中创建一个 `mod.rs` 文件，用来导出 `setup` 函数

```rust
// tests/common/mod.rs

pub fn setup() {
    println!("setup");
}
```


```rust
// tests/integration_test.rs

use Test::add;

mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(10, 0), 10);
    assert_eq!(add(-1, -2), -3);
}
```

```
Running tests/integration_test.rs (target/debug/deps/integration_test-b9b7236f69a53017)
```