use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    hash_map();
    hash_set();
}

fn hash_map() {
        let mut map = HashMap::new();

    println!("Is HashMap empty? {}", map.is_empty());

    map.insert(String::from("Yellow"), 10);
    map.insert(String::from("Blue"), 20);

    let value = map.get(&String::from("Yellow")).copied().unwrap_or(0);
    println!("The value of Yellow is: {}", value);

    map.insert(String::from("Yellow"), 20);

    let value = map.get(&String::from("Yellow")).copied().unwrap_or(0);
    println!("The value of Yellow is: {}", value);

    map.entry(String::from("Green")).or_insert(40); // 会被插入
    let value = map.get(&String::from("Green")).copied().unwrap_or(0);
    println!("The value of Green is: {}", value);

    map.entry(String::from("Blue")).or_insert(50); // 不会被插入，因为已经存在
    let value = map.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("The value of Blue is: {}", value); // 数据不会被覆盖

    let len = map.len();
    println!("HashMap length: {}", len);

    println!("Is HashMap empty? {}", map.is_empty());

    for (key, value) in &map {
        println!("key: {}, value: {}", key, value);
    }

    map.remove(&String::from("Blue"));
}

fn hash_set() {
    let mut set = HashSet::new();
    set.insert(1);

    println!("{}", set.contains(&1)); // 返回 true
    println!("{}", set.contains(&2)); // 返回 false

    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([2, 3, 4]);
    let diff = a.difference(&b);

    for x in diff {
        println!("{}", x);
    }

    println!();

    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([2, 3, 4]);
    let sym_diff = a.symmetric_difference(&b);

    for x in sym_diff {
        println!("{}", x);
    }
}