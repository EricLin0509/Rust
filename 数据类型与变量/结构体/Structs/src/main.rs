use std::cell::Cell;

fn main() {
    let student1  = Student {
        name: "Alice".to_string(),
        age: 20,
        score: Cell::from(95.0)
    };

    print_student(&student1);
    student1.score.set(85.0);
    print_student(&student1);

    let vehicle1 = Vehicle {
        manufacturer: "Porsche".to_string(),
        model: "911 GT3 RS 992.2".to_string(),
        year: 2021,
        color: Color::Silver
    };

    println!("{:?}", vehicle1);

    let vehicle2 = VehicleTuple("Hyundai".to_string(), "Elantra N".to_string(), 2021, Color::Blue);
    println!("{:?}", vehicle2);
    println!("{} {} {}", vehicle2.0, vehicle2.1, vehicle2.2);

    let person1 = init_person("Alice", 20);
    print_person(&person1);
    person1.name.set("Bob");
    print_person(&person1);
}

struct Student {
    name: String,
    age: i32,
    score: Cell<f64> // 使用 Cell 类型存储可变的 `score` 成员
}

fn print_student(student: &Student) {
    println!("Name: {}, Age: {}, Score: {}", student.name, student.age, student.score.get());
}

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: Color // 自定义类型
}

#[derive(Debug)]
struct VehicleTuple(String, String, u16, Color);


#[derive(Debug)]
enum Color {
    Silver,
    Blue,
    Red,
    Green,
    Yellow
}

struct Person<'p> {
    name: Cell<&'p str>, // 使用 Cell 类型存储可变的 `name` 成员
    age: u8
}

fn init_person<'a>(name: &'a str, age: u8) -> Person<'a> {
    let person = Person {
        name: Cell::from(name),
        age
    };
    person
}

fn print_person(person: &Person) {
    println!("Name: {}, Age: {}", person.name.get(), person.age);
}