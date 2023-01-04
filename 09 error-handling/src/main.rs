#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
      let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    // unwrap = get or panic
    // let greeting_file = File::open("hello.txt").unwrap();
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Problem creating the file: {:?}", error),
            // ErrorKind::NotFound => match File::create("hello.txt") {
            //     Ok(fc) => fc,
            //     Err(e) => panic!("Problem creating the file: {:?}", e),
            // },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
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

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)
}

fn read_username_from_file_suver_short() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

