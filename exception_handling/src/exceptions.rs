pub fn panic_demo() {
    panic!("panic is caused...");
}

pub fn vector_panic_demo() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    //v[99]; ==> this will give compile error
}

use std::fs::File;
use std::io::ErrorKind;

pub fn file_error() {
    let file_result = File::open("hello.txt");

    let file1 = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("issue creating the file {e:?}"),
            },
            _ => {
                panic!("problem opening of file {error:?}")
            }
        },
    };
}

pub fn unwrap_else() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
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

pub fn error_handling_using_match() {
    let result = read_username_from_file();
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
}

///////////////////////////
///
///
fn divide(num: f64, denom: f64) -> Result<f64, String> {
    if denom == 0.0 {
        Err(String::from("denom is zero"))
    } else {
        Ok(num / denom)
    }
}

fn using_question(file_name: &str) -> Result<String, io::Error> {
    let contents = std::fs::read_to_string(file_name)?; // The ? propagates the error if read fails
    Ok(format!("File contents: {}", contents))
}

//chaining method
fn read_username_from_file_chaining() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn using_result() {
    let mut result = divide(10.0, 2.0);
    match (result) {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }

    result = divide(10.0, 0.0);
    match (result) {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }

    //////
    /// using is_ok and is_err
    println!("using is_ok / is_err");
    result = divide(10.0, 2.0);
    if result.is_ok() {
        println!("result: {}", result.unwrap());
    }

    result = divide(10.0, 0.0);
    if result.is_err() {
        println!("error: {}", result.unwrap_err());
    }

    //using_question ? for propagating error.
    //file not existing..
    let mut result = using_question("test.txt");
    if result.is_ok() {
        println!("result: {}", result.unwrap());
    } else {
        println!("error: {}", result.unwrap_err());
    }

    //create the file
    std::fs::write("test.txt", "Hello from file!").expect("Could not write test file");
    result = using_question("test.txt");
    if result.is_ok() {
        println!("result: {}", result.unwrap());
    } else {
        println!("error: {}", result.unwrap_err());
    }
}
