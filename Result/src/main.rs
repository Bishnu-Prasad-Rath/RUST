//Result is Enum that can return either success or error because Result has the generic power.
use std::fs::File;

fn main() {
    let r = match divide(4, 1) {
        Ok(num) => num,
        Err(err) => -1,
    };
    println!("R = {r:#?}");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file : {e:?}"),
            },
            _ => panic!("Something went wrong {error:?}"),
        },
    };
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Please do not divide by error"));
    }
    Ok(x / y)
}
