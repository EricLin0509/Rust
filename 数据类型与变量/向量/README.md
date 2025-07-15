# 向量

向量 (Vector) 是一种线性数据结构，它可以存储一组相同类型的数据,可以改变大小

##  语法

```rust
let 变量名: Vec<数据类型> = Vec::new();
let 变量名: Vec<数据类型> = vec![值1, 值2, 值3,...];
```

## 代码示例

### 创建空向量

```rust
let v: Vec<i32> = Vec::new();
```

### 添加元素

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```

### 查看向量长度

使用 `len()` 方法可以查看向量的长度

```rust
let v: Vec<i32> = vec![1, 2, 3];
println!("{}", v.len()); // 输出: 3
```

### 查看向量容量

向量的容量**与长度不同**，容量是指**向量可以存储的元素数量**，而长度是指向量中实际存储的元素数量

向量的容量是根据向量的长度以 4 x 2<sup>n</sup> 增长的，且向量的容量永远大于等于向量的长度

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
println!("{}", v.capacity()); // 输出: 4
```

### 随机访问元素

使用索引值可以随机访问向量中的元素

```rust
let v: Vec<i32> = vec![1, 2, 3];
println!("{}", v[0]); // 输出: 1
```

现在，我们想获取一组元素的切片

```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5];
println!("{:?}", v[0..=1]);
```

此时会报错

```
the size for values of type `[i32]` cannot be known at compilation time
the trait `Sized` is not implemented for `[i32]
```

原因是 Rust 编译器无法确定 `[i32]` 的大小，因为它是一个未知的类型

为解决这个问题，我们需要使用 `as_slice()` 方法将向量转换为切片

```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5];
println!("{:?}", &(&v).as_slice()[0..=1]);
```

### 安全访问元素

如果向量的索引值越界，会导致程序 panic

为了避免这种情况，可以使用 `get()` 方法来安全访问元素

```rust
let v: Vec<i32> = vec![1, 2, 3];
if let Some(x) = v.get(2) {
    println!("{}", x);
} else {
    println!("Index out of bounds");
}
```

此时，如果索引值越界，`get()` 方法会返回 `None`，我们需要使用 `if let` 来判断是否存在元素


### 遍历向量

```rust
let v: Vec<i32> = vec![1, 2, 3];
for i in v {
    println!("{}", i);
}
```

但是这样有个问题，这样遍历 `v` 会导致 `v` 的所有权被转移

我们可以使用 `clone()` 方法来克隆 `v`

```rust
let v: Vec<i32> = vec![1, 2, 3];
for i in v.clone() {
    println!("{}", i);
}
```

或者创建一个切片

```rust
let v: Vec<i32> = vec![1, 2, 3];
for i in v.as_slice() {
    println!("{}", i);
}
```

### 合并向量

使用 `append()` 方法可以将两个向量集合合并

```rust
let mut v1: Vec<i32> = vec![1, 2, 3];
let mut v2: Vec<i32> = vec![4, 5, 6];
v1.append(&mut v2);
```

**注意：合并后 `v2` 会被清空**

### 插入元素

与 `push()` 相比，`insert()` 方法可以指定插入位置

```rust
let mut v: Vec<i32> = vec![1, 2, 4];
v.insert(2, 3); // 在第 2 个位置插入 3
```

### 删除元素

使用 `remove()` 方法可以删除指定位置的元素

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
v.remove(1); // 删除第 2 个元素
```

### 带条件删除元素

使用 `retain()` 方法可以删除不满足条件的元素

```rust
let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
v.retain(|e: &i32| if *e % 2 == 0 { true } else { false });
```

其中，`retain` 的参数是一个闭包，格式如下

```
FnMut(&T) -> bool
```

**当条件为 `true` 时，保留元素；当条件为 `false` 时，删除元素**

### 预分配空间

使用 `reserve()` 方法可以预分配空间

```rust
let mut v: Vec<i32> = Vec::new();
v.reserve(10); // 预分配 10 个元素的空间
```