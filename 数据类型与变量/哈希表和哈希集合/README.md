# 哈希表

哈希表 (HashMap) 是一种数据结构，它通过把键映射到值来存储数据。它是通过哈希函数将键转换为索引，然后将值存储在该索引处

## 语法

```rust
let mut 变量名 = HashMap::new();
```

- 通常来说，哈希表是可变的，因为你可能需要添加或删除键值对

## 代码示例

### 引入 HashMap 库

需要引入 `std::collections::HashMap` 库

```rust
use std::collections::HashMap;
```

### 创建 HashMap

可以不用指定键值对的类型，因为 Rust 可根据插入的类型推断类型

```rust
let mut map = HashMap::new();
```

如果只创建一个空的 HashMap，则需要指定键值对的类型

```rust
let mut map: HashMap<String, i32> = HashMap::new();
```

### 获取 HashMap 的长度

使用 `len` 方法获取 HashMap 的长度

```rust
let len = map.len();
println!("HashMap length: {}", len);
```

### 检查是否为空

使用 `is_empty` 方法检查是否为空

```rust
println!("Is HashMap empty? {}", map.is_empty());
```

### 添加键值对

插入格式为 `HashMap<K, V>`

- `K` 是键的类型
- `V` 是值的类型

```rust
map.insert(String::from("Yellow"), 10);
map.insert(String::from("Blue"), 20);
```

### 获取值

首先需要使用 `get` 方法获取值，如果键存在，则返回 `Some(&V)`，否则返回 `None`

```rust
let value = map.get(&String::from("Yellow"));
```

- **注意：`get` 参数是一个泛型引用 `&K`**

**由于 `get` 方法返回的是 `Option<&V>` 类型**，所以需要使用 `copied` 方法将 `Option<&V>` 转换为 `Option<V>`

```rust
let value = map.get(&String::from("Yellow")).copied();
```

最后使用 `unwrap_or` 方法获取值，如果键不存在，则返回默认值

```rust
let value = map.get(&String::from("Yellow")).copied().unwrap_or(0);
```

- 这里使用 `unwrap_or` 方法而不是 `unwarp` 方法，因为如果返回 `None`，会发生 panic

### 遍历 HashMap

使用 `for` 循环遍历 HashMap

```rust
for (key, value) in &map {
    println!("key: {}, value: {}", key, value);
}
```

- 这里使用 `&map` 而不是 `map`，因为在遍历时，`map` 中的键值对的所有权将被转移

### 删除键值对

使用 `remove` 方法删除键值对

- **注意：`remove` 方法的参数是一个泛型引用 `&K`**

```rust
map.remove(&String::from("Blue"));
```

### 更新值

使用 `insert` 方法更新值

```rust
map.insert(String::from("Yellow"), 30);
```

### 仅插入不存在的键

哈希表有一个 `entry` 方法，它用于检查键是否存在，并返回一个名为 `Entry` 的枚举，该枚举表示传入的键是否存在于哈希表中

```rust
map.entry(String::from("Green"))
```

其中 `Entry` 的 `or_insert` 方法用于插入不存在的键，并返回其值

```rust
map.entry(String::from("Green")).or_insert(40);
```

# 哈希集合

哈希集合 (HashSet) 是一种数据结构，它是由哈希表实现的，只存储键，不存储值。它通过哈希函数将键转换为索引，然后将键存储在该索引处

- 如果键**已经存在于集合中，则不会重复插入**

## 语法

```rust
let mut 变量名 = HashSet::new();
```

- 通常来说，哈希集合是可变的，因为你可能需要添加或删除键

## 代码示例

### 引入 HashSet 库

需要引入 `std::collections::HashSet` 库

```rust
use std::collections::HashSet;
```

### 创建 HashSet

可以不用指定键的类型，因为 Rust 可根据插入的类型推断类型

```rust
let mut set = HashSet::new();
```

如果只创建一个空的 HashSet，则需要指定键的类型

```rust
let mut set: HashSet<String> = HashSet::new();
```

### 获取 HashSet 的长度

使用 `len` 方法获取 HashSet 的长度

```rust
let len = set.len();
println!("HashSet length: {}", len);
```

### 检查是否为空

使用 `is_empty` 方法检查是否为空

```rust
println!("Is HashSet empty? {}", set.is_empty());
```

### 添加键

插入格式为 `HashSet<K>`

- `K` 是键的类型

```rust
set.insert(String::from("Yellow"));
set.insert(String::from("Blue"));
```

#### 添加一组建

使用 `from` 方法将一组键转换为 HashSet

```rust
let set = HashSet::from([1, 2, 3]);
```

### 检查键是否存在

使用 `contains` 方法检查键是否存在

```rust
println!("{}", set.contains(&1));
```

### 比较两个集合

使用 `difference` 方法比较两个集合的差集

```rust
let a = HashSet::from([1, 2, 3]);
let b = HashSet::from([2, 3, 4]);
let diff = a.difference(&b);

for x in diff {
    println!("{}", x);
}
```

- 使用 `difference` 方法只会返回 `a` 集合不包含在 `b` 集合中的元素

使用 `symmetric_difference` 方法比较两个集合的对称差集

```rust
let a = HashSet::from([1, 2, 3]);
let b = HashSet::from([2, 3, 4]);
let sym_diff = a.symmetric_difference(&b);

for x in sym_diff {
    println!("{}", x);
}
```

- 使用 `symmetric_difference` 方法会返回 `a` 集合和 `b` 集合的对称差集，即 `a` 集合和 `b` 集合的并集减去交集