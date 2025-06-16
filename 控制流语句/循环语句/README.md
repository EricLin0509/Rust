# 循环语句

## 简介

在 Rust 中，循环语句分为两种：`loop`、`while` 和 `for`

- `loop`：无限循环，直到 `break` 语句被执行
- `while`：条件循环，循环条件为布尔表达式
- `for`：迭代器循环，循环遍历集合中的元素

## 代码示例

### `loop` 循环

```rust
loop {
    // 循环体
}
```

`loop` 循环无限循环，直到 `break` 语句被执行

```rust
int mut count = 0;

loop {
    if count == 10 {
        break;
    }
    println!("hello world");
    count += 1;
}
```

### `while` 循环

```rust
while 条件表达式 {
    // 循环体
}
```

`while` 循环条件循环，循环条件为布尔表达式

```rust
int mut count = 0;

while count < 10 {
    println!("hello world");
    count += 1;
}
```

### `for` 循环

```rust
for 变量 in 集合 {
    // 循环体
}
```

`for` 循环迭代器循环，循环遍历集合中的元素 (如数组，元组，集合等)

```rust
let ages = [18, 16, 20, 17, 25];
let mut count = 0;

for age in ages {
    if age >= 18 {
        count += 1;
    }
}
println!("有 {} 人可以考驾照", count);
```

### 循环中断

有两种方式可以中断循环：`break` 和 `continue`

- `break`：终止当前循环，并从循环体外继续执行
- `continue`：跳过当前迭代，继续执行下一次迭代

例如当 `age` 小于 18 时，`continue` 跳过当前迭代，继续执行下一次迭代

```rust
let ages = [18, 16, 20, 17, 25];

for age in ages {
    if age < 18 {
        continue;
    }
    println!("{} 可以考驾照", age);
}
```

```
18 可以考驾照
20 可以考驾照
25 可以考驾照
```

当 `count` 等于 5 时，`break` 终止当前循环

```rust
int mut count = 0;

loop {
    if count == 5 {
        break;
    }
    println!("hello world");
    count += 1;
}
```

```
hello world
hello world
hello world
hello world
hello world
```