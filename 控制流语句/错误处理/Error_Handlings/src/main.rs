use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let read_data = File::open("data.txt");

    match read_data {
        Ok(file) => {
            println!("File opened successfully")
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            other_error => println!("Unknown error: {}", other_error),
        },
    };

    read_name_from_file();
}

fn read_name_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("name.txt")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}