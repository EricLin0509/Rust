# 结构体

结构体是一种自定义的数据类型，可以包含多个不同的数据类型，可以定义方法和属性

## 语法

```rust
struct 结构体名 {
    成员1: 类型1,
    成员2: 类型2,
    ...
    成员n: 类型n
}
```

## 代码示例

### 定义结构体

假设现在要定义一个学生的数据结构，包含姓名、年龄、成绩三个成员，分别为字符串、整数、浮点数

```rust
struct Student {
    name: String,
    age: i32,
    score: f64
}
```

**注意：如果在模块中定义结构体，需要在结构体前添加 `pub` 关键字**

```rust
pub struct Student {
    pub name: String, // 要访问结构体的成员，也需要添加 pub 关键字
    pub age: i32,
    pub score: f64
}
```

### 创建结构体实例

```rust
let student1 = Student {
    name: "Alice".to_string(),
    age: 20,
    score: 90.5
};
```

### 访问结构体成员

**使用 `结构体名.成员名` 访问成员**

```rust
println!("Name: {}", student1.name);
println!("Age: {}", student1.age);
println!("Score: {}", student1.score);
```

### 自定义成员

成员类型可以是任何有效的 Rust 类型，包括自定义类型

```rust
enum Color {
    Silver,
    Blue,
    Red,
    Green,
    Yellow
}

struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: Color // 自定义类型
}
```

### 调试结构体

```rust
let student1 = Student {
    name: "Alice".to_string(),
    age: 20,
    score: 90.5
};

println!("{:?}", student1);
```

此时会报错

```
`Vehicle` doesn't implement `Debug`
the trait `Debug` is not implemented for `Vehicle`
```

这是因为 `Vehicle` 结构体没有实现 `Debug` trait，需要手动实现 `Debug` trait，使得结构体可以被 `println!` 输出

使用 `#[derive(Debug)]` 宏可以自动实现 `Debug` trait

```rust
#[derive(Debug)] // 自动实现 Debug trait
struct Student {
    name: String,
    age: i32,
    score: f64
}
```

### 元组结构体

元组结构体可以包含不同类型的数据，但只能作为函数的返回值，**不能作为结构体的成员**

```rust
struct VehicleTuple(String, String, u16, Color);

let vehicle = VehicleTuple("Hyundai".to_string(), "Elantra N".to_string(), 2021, Color::Blue);
```

#### 访问元组结构体成员

```rust
println!("Manufacturer: {}", vehicle.0);
println!("Model: {}", vehicle.1);
println!("Year: {}", vehicle.2);
println!("Color: {:?}", vehicle.3);
```

### 可变结构体

结构体默认是不可变的，如果需要修改结构体成员，需要使用 `mut` 关键字

```rust
let mut student1 = Student {
    name: "Alice".to_string(),
    age: 20,
    score: 90.5
};

student1.name = "Bob".to_string(); // `name` 成员被修改
```

但是，这样会让整个结构体变为可变

也不能直接在定义结构体时使用 `mut` 关键字，因为结构体的成员默认是不可变的

```rust
struct Student {
    name: String,
    age: i32,
    mut score: f64 // 错误：不能在定义结构体时使用 `mut` 关键字
}
```

这时只能使用 Cell 类型，Cell 类型可以用来指定哪些成员是可变的

### Cell 类型

Cell 类型可以用来指定哪些成员是可变的，Cell 类型可以存储 `&mut T` 类型的值，但不能存储 `&T` 类型的值

```rust
pub struct Cell<T>
where
    T: ?Sized,
```


#### 引入 Cell 类型

```rust
use std::cell::Cell;
```

#### 修改结构体成员

```rust
struct Student {
    name: String,
    age: i32,
    score: Cell<f64> // 使用 Cell 类型存储可变的 `score` 成员
}
```

**注意：如果是String类型，不能使用Cell类型 (因为String类型没有实现 Copy trait)，要这样写:**

```rust
struct Student<'p> {
    name: Cell<& 'p str>,
    age: i32,
    score: u16
}
```

- `'p` —— 生命周期参数，表示 `name` 成员的生命周期
- `& 'p str` —— 引用类型，指向生命周期参数 `'p` 的字符串

#### 初始化结构体成员

如果是 Cell 类型，则需要使用 `from` 方法初始化成员

```rust
let student1 = Student {
    name: "Alice".to_string(),
    age: 20,
    score: Cell::from(90.5)
};
```

#### 修改结构体成员

如果使用 `from` 方法修改成员

```rust
student1.score = Cell::from(85.0);
```

此时会报错

```
cannot assign to `student1.score`, as `student1` is not declared as mutable
cannot assign
```

这是因为结构体 `student1` 本身不是可变的，不能直接修改成员

如果需要修改 Cell 类型成员，就需要使用 Cell 类型提供的 `set` 方法

```rust
student1.score.set(85.0);
```

### 获取 Cell 类型的值

如果需要获取 Cell 类型的值，则可以使用 `get` 方法

```rust
println!("Score: {}", student1.score.get());
```