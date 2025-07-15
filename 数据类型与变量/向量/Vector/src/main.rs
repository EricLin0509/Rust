fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);
    println!("{}", v.len()); // 输出: 3
    println!("{}", v.capacity()); // 输出: 4

    println!("{}", v[0]); // 输出: 1
    
    println!("{:?}", &(&v).as_slice()[0..=1]);

    if let Some(x) = v.get(2) {
        println!("{}", x);
    } else {
        println!("Index out of bounds");
    }

    for i in v.clone() {
        println!("{}", i);
    }

    let mut v1: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<i32> = vec![4, 5, 6];
    v1.append(&mut v2);
    println!("{:?}", v1);
    println!("{:?}", v2);

    v.insert(2, 3);
    println!("{:?}", v);

    v1.retain(|e: &i32| if *e % 2 == 0 { true } else { false });
    println!("{:?}", v1);
}
