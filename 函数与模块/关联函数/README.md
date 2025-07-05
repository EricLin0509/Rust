# 关联函数

在 Rust 中，关联函数 (Associated Functions) 是一种特殊的函数，它可以被定义在结构体、枚举、trait 或模块中，并与该类型相关联

## 语法

```rust
impl 类型 {
    fn 函数名(参数列表) -> 返回值类型 {
        函数体
    }    
}
```

## 代码示例

假设现在有一个 `Vehicle` 结构体

```rust
struct Vehicle<'p> {
    manufacturer: String,
    model: Cell<&'p str>,
    year: u32,
}
```

现在，我们可以为 `Vehicle` 实现一个关联函数 `change_model` 来修改 `model` 字段

```rust
impl<'p> Vehicle<'p> {
    fn change_model(&self, new_model: &'p str) {
        self.model.set(new_model);
    }
}
```

- `impl` 关键字用于定义关联函数的实现
- `self` 参数是关联函数的第一个参数，它代表了调用该函数的实例

所以，`self` 代表了 `Vehicle` 实例，**只能通过 `Vehicle` 实例来调用 `change_model` 函数**

### 调用关联函数

使用 `对象名.函数名(参数列表)` 来调用关联函数

```rust
let mut my_car = Vehicle {
    manufacturer: "Toyota".to_string(),
    model: Cell::new("Corolla"),
    year: 2021,
};

my_car.change_model("Camary"); // 调用关联函数
```

### 使用可变借用

要使用可变借用，将 `&mut self` 作为关联函数的第一个参数

```rust
fn change_year(&mut self, new_year: u32) {
    self.year = new_year;
}
```

这样，就可以修改 `Vehicle` 实例的 `year` 字段了

```rust
my_car.change_year(2022);
```

### 静态方法

静态方法 (Static Methods) 是一种特殊的关联函数，它可以被定义在结构体、枚举或模块中，但不能访问实例的任何字段

```rust
fn create_vehicle(manufacturer: &str, model: &str, year: u32) -> Vehicle {
    return Vehicle {
        manufacturer: manufacturer.to_string(),
        model: Cell::new(model),
        year,
    };
}
```

```rust
let my_car = Vehicle::create_vehicle("Toyota", "Camry", 2022); // 调用静态方法
```

静态方法不能访问实例的任何字段，所以不能修改 `Vehicle` 实例的任何字段