fn main() {
    let mut count = 0;

    loop {
        if count == 10 {
            break;
        }
        println!("hello world");
        count += 1;
    }

    count = 0;

    while count < 10 {
        println!("hello world");
        count += 1;
    }

    let ages = [18, 16, 20, 17, 25];
    let mut count = 0;

    for age in ages {
        if age >= 18 {
            count += 1;
        }
    }
    println!("有 {} 人可以考驾照", count);
}
