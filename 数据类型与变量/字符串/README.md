# 字符串

在 Rust 中，字符串是其中一种集合类型。字符串是一系列的 Unicode 字符，它可以包含任意数量的字符，包括空格、数字、字母、符号等。字符串是可变的，这意味着你可以修改字符串的内容

## 代码示例

### 定义字符串

```rust
let s1 = String::new(); // 空字符串
let s2 = "Hello world"; // 字符串切片
let s3 = s2.to_string();
let s4 = String::from("Hello world"); // 通过 String::from() 方法创建字符串
```

### 添加字符/字符串

- 使用 `push()` 方法添加单个字符到字符串末尾
- 使用 `push_str()` 方法添加字符串切片到字符串末尾

```rust
let mut string = String::from("Hello ");
string.push('w');
string.push_str("orld!");
```

也可以使用 `+` 运算符来拼接字符串，虽说可以减少内存使用 (不需要复制字符串)，但有可能会导致所有权转移 (如果不使用引用类型)

```rust
let s1 = String::from("Hello ");
let s2 = String::from("world!");

let s3 = s1 + &s2;
```

- `s1` 的所有权被转移给 `s3`
- `s2` 的所有权被保留

也可以使用 `format!` 宏来格式化字符串

```rust
let s1 = String::from("Hello ");
let s2 = String::from("world!");

let s3 = format!("{}{}", s1, s2);
```

- `s1` 和 `s2` 的所有权被保留

### 访问字符串

尝试使用索引访问字符串中的字符

```rust
let s = "你好，世界！";
let c = s[0]; // 第一个字符
```

此时会报错

```
error[E0277]: the type `str` cannot be indexed by `{integer}`
```

这是因为 Unicode 字符可以占用 1~4 个字节，如果直接使用索引访问，可能会导致无法正确访问到字符 (因为索引是以字节为单位的)

所以，Rust 提供了三个方法来访问字符串中的字符：

- `chars()` 方法：返回一个迭代器，可以用来遍历字符串中的字符
- `bytes()` 方法：返回一个迭代器，可以用来遍历字符串中的字节
- `graphemes()` 方法：返回一个迭代器，可以用来遍历字符串中的 Unicode 组成单元 (grapheme)
    - **注意：使用 `graphemes()` 方法，需要引用 `unicode-segmentation` 库**

```rust
for c in "你好，世界！".chars() {
    println!("{}", c);
}
```

```rust
for b in "你好，世界！".bytes() {
    println!("{}", b);
}
```

```rust
use unicode_segmentation::UnicodeSegmentation; // 引用 unicode-segmentation 库

for g in "你好，世界！".graphemes(true) {
    println!("{}", g);
}
```
