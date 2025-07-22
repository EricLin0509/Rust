fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 创建迭代器
    let iter = numbers.iter();

    // 求和
    let sum: i32 = iter.clone().sum(); // 必须指定元素类型
    println!("{}", sum); // 15

    let mapped_iter = iter.map(|x| x * 2); // [2, 4, 6, 8, 10]

    for i in mapped_iter {
        print!("{} ", i);
    }
    println!();

    let counter = Counter::new(1, 10, 2);
    for i in counter {
        print!("{} ", i); // 1, 3, 5, 7, 9
    }
    println!();
}

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