struct Person<PetType: Animal> {
    name: String,
    pet: PetType,
}

trait Animal {
    fn eat(&self);
}

trait Sound {
    fn speak(&self);
}

struct Dog {
    name: String,
    age: u8,
}

impl Animal for Dog { // Dog 结构体实现 Animal 特征
    fn eat(&self) {
        println!("Dog is eating");
    }
}

impl Sound for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
struct Cat {
    name: String,
    age: u8,
}

impl Animal for Cat { // Cat 结构体实现 Animal 特征
    fn eat(&self) {
        println!("Cat is eating");
    }
}

impl Sound for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

pub fn new_person() {
    let pet1 = Dog { name: "Rufus".to_string(), age: 3 };
    let person1 = Person { name: "Alice".to_string(), pet: pet1 };
    person1.pet.eat();
    person1.pet.speak();
    
    let pet2 = Cat { name: "Fluffy".to_string(), age: 5 };
    let person2 = Person { name: "Bob".to_string(), pet: pet2 };
    person2.pet.eat();
    person2.pet.speak();
}