# 错误处理

Rust 有一个内置的错误处理机制，它可以帮助我们处理各种类型的错误

- `Result<T, E>` —— 用于可以恢复的错误，比如文件读取失败、网络连接失败等

## 代码示例

### panic!

`panic!` 宏会导致程序的崩溃，并打印出一个错误消息和调用栈。它可以用于触发不可恢复的错误

```rust
fn main() {
    panic!("This is a panic!");
}
```

此时程序会打印出如下错误信息：

```
thread 'main' panicked at src/main.rs:2:5:
This is a panic!
```

### Result枚举

`Result` 枚举可以用来表示函数的返回值，其中 `Ok` 表示函数执行成功，`Err` 表示函数执行失败

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 一般用于可以恢复的错误，比如文件读取失败、网络连接失败等

```rust
use std::fs::File;

let read_data = File::open("data.txt");

match read_data {
    Ok(file) => {
        println!("File opened successfully")
    },
    Err(error) => {
        println!("Error opening file: {}", error)
    },
};
```

- 使用 `match` 表达式来处理 `Result` 枚举
- `Ok` 匹配成功的情况，`Err` 匹配失败的情况
- 在 `Err` 匹配的情况下，可以返回错误信息，并退出程序

### 错误类型

使用 `kind()` 方法可以获取错误的类型 (需要导入 `std::io::ErrorKind` 模块)

```rust
use std::io::ErrorKind; // 导入 ErrorKind

match read_data {
    Ok(file) => {
        println!("File opened successfully")
    },
    Err(error) => match error.kind() {
        ErrorKind::NotFound => println!("File not found"),
        ErrorKind::PermissionDenied => println!("Permission denied"),
        other_error => println!("Unknown error: {}", other_error),
    },
};
```

- `ErrorKind` 枚举包含了各种类型的错误
   - `NotFound`：文件不存在
   - `PermissionDenied`：没有权限访问文件
   - `ConnectionRefused`：连接被拒绝
   - `ConnectionReset`：连接被重置
   - ......


### 指定错误信息

使用 `expect()` 方法可以指定错误信息

```rust
let read_data = File::open("data.txt").expect("Failed to open \"data.txt\"");
```

```
thread 'main' panicked at src/main.rs:5:44:
Failed to open "data.txt"
```

- `expect()` 方法会在 `Err` 匹配的情况下，打印指定的错误信息，并退出程序

### 简化错误处理

使用 `?` 运算符可以简化错误处理代码，它可以自动匹配 `Result` 枚举，并返回 `Ok` 值，如果 `Result` 枚举是 `Err`，则会调用 `panic!` 宏并打印错误信息

假设我们现在有如下函数：

```rust
use std::fs::Read;

fn read_name_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("name.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => Err(error),
    }
}
```

我们可以用 `?` 运算符简化代码：

```rust
fn read_name_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("name.txt")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
```