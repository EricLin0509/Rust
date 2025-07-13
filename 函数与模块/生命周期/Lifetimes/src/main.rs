fn main() {
    let result;
    let x = String::from("hello");
    {
        let y = String::from("world");
        result = longest_str(&x, &y);
        println!("result: {}", result);
    }
}

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Example<'a> {
    parts: &'a str
}

impl<'a> Example<'a> {
    fn return_parts(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.parts
    }
}