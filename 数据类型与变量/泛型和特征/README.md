# 特征

在 Rust 中，特征 (Traits) 是一种抽象概念，它定义了某种行为或属性，可以被其他类型实现。类似于 Java 中的接口 (Interface)

特征可以用于指定泛型函数、结构体、枚举、方法的行为，也可以用于约束泛型类型参数

## 语法

```rust
trait 特征名 {
    // trait 的方法定义
}
```

## 代码示例

假设现在有一个 `Pet` 模块，里面有 `Person` 、 `Dog` 、 `Cat` 三个结构体

- `Person` 结构体用于表示人类，有 `pet` 字段
- `Dog`、`Cat` 结构体用于表示狗和猫，作用于 `Person` 结构体的 `pet` 字段

```rust
// Pet.rs

struct PersonWithDog {
    name: String,
    pet: Dog,
}

struct PersonWithCat {
    name: String,
    pet: Cat,
}

struct Dog {
    name: String,
    age: u8,
}

struct Cat {
    name: String,
    age: u8,
}
```

但是这样有一个问题，需要创建两个 `Person` 结构体，分别代表人类和狗，或者人类和猫。如果有更多的动物类型，就需要创建更多的结构体，使得代码变得繁琐

为了解决这个问题，可以使用特征 (Trait) 来定义 `Animal` 特征，并在 `Dog` 和 `Cat` 结构体上实现该特征

### 定义特征

```rust
// Pet.rs

trait Animal {    } // 定义 Animal 特征
```

### 实现特征

```rust
// Pet.rs

impl Animal for Dog {    } // Dog 结构体实现 Animal 特征
impl Animal for Cat {    } // Cat 结构体实现 Animal 特征
```

### 使用特征

这时，我们可以将 `Animal` 特征作为泛型参数，在 `Person` 结构体中使用它

```rust
struct Person<PetType: Animal> {
    name: String,
    pet: PetType,
}

pub fn new_person() {
    let pet1 = Dog { name: "Rufus".to_string(), age: 3 };
    let person1 = Person { name: "Alice".to_string(), pet: pet1 };
    
    let pet2 = Cat { name: "Fluffy".to_string(), age: 5 };
    let person2 = Person { name: "Bob".to_string(), pet: pet2 };
}
```

### 设置方法

现在，我们可以为 `Animal` 特征定义方法，比如 `eat` 方法，用于表示动物吃东西

```rust
// Pet.rs

trait Animal {
    fn eat(&self);
}

impl Animal for Dog {
    fn eat(&self) {
        println!("Dog is eating");
    }
}

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat is eating");
    }
}
```

### 实现多个特征

假设现在有一个 `Sound` 特征，它定义了 `speak` 方法，用于表示动物发出声音

```rust
// Pet.rs

trait Animal {
    fn eat(&self);
}

trait Sound {
    fn speak(&self);
}

impl Animal for Dog { // Dog 结构体实现 Animal 特征
    fn eat(&self) {
        println!("Dog is eating");
    }
}

impl Sound for Dog { // Dog 结构体实现 Sound 特征
    fn speak(&self) {
        println!("Woof!");
    }
}


impl Animal for Cat { // Cat 结构体实现 Animal 特征
    fn eat(&self) {
        println!("Cat is eating");
    }
}

impl Sound for Cat { // Cat 结构体实现 Sound 特征
    fn speak(&self) {
        println!("Meow!");
    }
}
```


# 泛型

在 Rust 中，泛型 (Generics) 是指在定义函数、结构体、枚举、方法等时，可以将类型参数 (Type Parameters) 作为占位符，在编译时再指定具体的类型

## 语法

泛型函数、结构体、枚举、方法的定义语法如下：

```rust
fn 函数名<泛型参数>(参数列表) -> 返回值类型 {
    函数体
}

struct 结构体名<泛型参数> {
    字段列表
}

enum 枚举名<泛型参数> {
    成员列表
}

impl<泛型参数> 结构体名<泛型参数> {
    方法定义
}
```

## 代码示例

```rust
struct Demo <T> {
    data: T,
}
```

- `T` 是泛型参数，可以用任何类型来替换

### 约束

泛型参数可以有约束，比如 `T: u8` 表示 `T` 必须是一个 `u8` 类型

```rust
struct Demo <T: u8> {
    data: T,
}
```

### 混合约束

多个泛型参数可以有多个约束，比如 `T: Display + Clone` 表示 `T` 必须实现 `Display` 和 `Clone` 两个特征

```rust
struct Demo <T: Display + Clone> {
    data: T,
}
```

或者使用 `where` 关键字来简化约束

```rust
struct Demo <T> where T: Display + Clone {
    data: T,
}
```