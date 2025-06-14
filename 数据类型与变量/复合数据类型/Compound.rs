fn main() {
    let tuple = ("Alice", 30, true);
    println!("{:?}", tuple);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let slice1 = &a[1..3];
    println!("{:?}", slice1);

    let slice2 = &a[1..=3];
    println!("{:?}", slice2);
}
