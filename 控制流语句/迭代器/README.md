# 迭代器

Rust 中的迭代器是一种高级的概念，它允许我们遍历集合 (如向量、哈希表) 中的元素，而无需手动管理索引。迭代器提供了一种更安全、更简洁的方式来处理集合

## 语法

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 创建迭代器
let iter = numbers.iter();
```

## 代码示例

### 创建迭代器

使用 `iter()` 方法可以创建迭代器

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 创建迭代器
let iter = numbers.iter();
```

### 遍历迭代器

可以使用 `for` 循环遍历迭代器

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 创建迭代器
let iter = numbers.iter();

// 遍历迭代器
for num in iter {
    println!("{}", num);
}
```

### 原理

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

- `Item` 是一个泛型类型参数，表示迭代器返回的元素类型

- `self` 是 `Iterator` 类型的实例
    - 它必须是可变的，因为迭代器可能需要修改内部状态以生成下一个元素

- `next()` 方法返回 `Option<Self::Item>` 类型，其中 `Some(item)` 表示迭代器返回下一个元素，`None` 表示迭代器已经遍历完毕


### 迭代器的分类

如果我们想要迭代器是可变的，则需要使用 `iter_mut()` 方法创建可变迭代器

```rust
let mut numbers = vec![1, 2, 3, 4, 5];

// 创建可变迭代器
let mut iter = numbers.iter_mut();
```

如果我们想要迭代器消耗集合生成元素，则需要使用 `into_iter()` 方法创建消费迭代器

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 创建消费迭代器
let iter = numbers.into_iter();

// for i in numbers // 错误，`numbers` 的所有权已被转移
```

### 求和

可以使用 `sum()` 方法求和

```rust
let sum: i32 = iter.sum(); // 必须指定元素类型
println!("{}", sum); // 15
```

- **注意：使用 `sum()` 求和的时候必须指定元素类型，否则会报错**
- **注意：`sum()` 方法会转移所有权**

### 映射

可以使用 `map()` 方法对迭代器元素进行映射

```rust
let mapped_iter = iter.map(|x| x * 2); // [2, 4, 6, 8, 10]
```

### 实现自己的迭代器

假设现在我们有一个 `Counter` 结构体，它有一个 `start` 属性、`end` 属性和 `step` 属性，我们希望实现一个迭代器，它可以生成 `start` 到 `end` 之间，步长为 `step` 的数字

```rust
struct Counter {
    start: u32,
    end: u32,
    step: u32,
}

impl Counter {
    fn new(start: u32, end: u32, step: u32) -> Counter {
        Counter { start, end, step }
    }
}
```

我们需要实现 `Iterator` 特征，并实现 `next()` 方法

```rust
impl Iterator for Counter {
    type Item = u32; // 元素类型

    fn next(&mut self) -> Option<Self::Item> { // 实现 next() 方法
        if self.start < self.end {
            let value = self.start;
            self.start += self.step;
            Some(value)
        } else {
            None
        }
    }
}
```