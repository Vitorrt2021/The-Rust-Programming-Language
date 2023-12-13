use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    try_panic();
    try_open_file();
}

fn force_panic(){
    panic!("crash and burn");
}


fn try_panic(){
    let v = vec![1, 2, 3];

    v[99];
}

fn try_open_file(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn unwrap_file(){
//    If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action
    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file_2 = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}

fn error_propagation() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn error_propagation_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn error_propagation_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn option_propagation(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}