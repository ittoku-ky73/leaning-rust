use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // match expretion
    {
        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Probrem opening the file: {:?}", other_error);
                }
            },
        };
    }

    // closures
    {
        let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Probrem opening the file: {:?}", error);
            }
        });
    }

    // unwrap
    {
        let _greeting_file = File::open("hello.txt").unwrap();
    }

    // ? oparator using main function
    {
        let _greeting_file = File::open("hello.txt")?;
        Ok(())
    }
}

// Read username from "hello.txt"
fn _read_username_from_file() -> Result<String, io::Error> {
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

// using ? operator
fn _read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// using ? operator shortly
fn _read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// return last char of first line
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
