use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(_) => {
            println!("found file hello.txt");
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(_) => {
                    println!("Created file hello.txt");
                },
                Err(error) => panic!("{}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}