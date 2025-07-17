enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coins) -> u32 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

fn main() {
    let coin = Coins::Quarter;
    let value = value_in_cents(coin);
    println!("You have {} cents", value);
}