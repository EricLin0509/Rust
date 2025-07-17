# 枚举

枚举 (enum) 是一种数据类型，它可以用来定义一组相关的变量，每个变量都有自己的名称和值。枚举可以用来代替常量，因为枚举可以包含更多的可能值，而且可以提供更好的类型检查

## 语法

```rust
enum 枚举名 {
    变量名1(数据类型1),
    变量名2,
    //...
}
```

- 如果变量名不存储任何值，则可以省略数据类型

## 代码示例

假设我们要添加一个 IPV4 地址和 IPV6 地址的数据类型，可以用枚举来实现：

```rust
enum IpAddress {
    V4,
    V6,
}

fn main() {
    let four = IpAddress::V4;
    let six = IpAddress::V6;
}
```

### 枚举与结构体配合使用

我们也可以与结构体配合使用，来存储 IP 地址的详细信息：

```rust
enum IpAddress {
    V4,
    V6,
}

struct IpAddressDetails {
    kind: IpAddress,
    address: String,
}

fn main() {
    let localhost = IpAddressDetails {
        kind: IpAddress::V4,
        address: "127.0.0.1".to_string(),
    };
}
```

### 指定枚举变量的数据类型

如果枚举变量的数据类型需要指定，则可以在变量名后面加上类型注解：

```rust
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let localhost = IpAddress::V4(127, 0, 0, 1);
    let ipv6 = IpAddress::V6("::1".to_string());
}
```

### 设置方法

枚举可以为每个变量设置方法，以提供额外的功能：

```rust
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddress {
    fn print_address(&self) {
        match self {
            IpAddress::V4(a, b, c, d) => println!("{}:{}:{}:{}", a, b, c, d),
            IpAddress::V6(s) => println!("{}", s),
        }
    }
}
```

### 模式匹配

枚举可以与模式匹配一起使用，来处理不同枚举变量的不同情况：

```rust
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coins) -> u32 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

fn main() {
    let coin = Coins::Quarter;
    let value = value_in_cents(coin);
    println!("You have {} cents", value);
}
```

同样的，这个也可以在选项 (Option) 类型中使用，来处理可能缺失的值：

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
        None => None,
    }
}
```