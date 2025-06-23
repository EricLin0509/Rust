fn main() {
    println!("Hello, world!");
    let mut x = test_option();
    x = Some(10);
    println!("x = {}", x.unwrap());
    let y = test_option_string();
    println!("y = {}", y.unwrap());

    let choice = choose_character();

    if choice.is_some() {
        println!("You chose: {}", choice.unwrap().to_string());
    } else {
        println!("You didn't choose a character.");
    }
}

fn test_option() -> Option<i32> {
    let x: Option<i32> = Some(5);
    return x;
}

fn test_option_string() -> Option<String> {
    return Some("Hello, world!".to_string());
}

fn choose_character() -> Option<Characters> {
    let choice = Some(Characters::Archer);
    return choice;
}

#[allow(dead_code)]
enum Characters {
    Archer,
    Knight,
    Mage,
}

impl ToString for Characters {
    fn to_string(&self) -> String {
        match self {
            Characters::Archer => "Archer",
            Characters::Knight => "Knight",
            Characters::Mage => "Mage",
        }.to_string()
    }
}