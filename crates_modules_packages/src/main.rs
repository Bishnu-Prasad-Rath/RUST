// src/main.rs  <-- Root Binary Crate
// src/lib.rs   <-- Root Lib Crate

//It's a good practice to store all the functionalities in lib.rs

use crates_modules_packages::Credentials; //It is used to get the resouce to a specific scope -> It is getting all the Credentials from crates_modules_packages
use crates_modules_packages::auth_utils::models::New;
use crates_modules_packages::authenticate;

fn main() {
    println!("Hello, world!");

    let cred = New {
        username: String::from("Bishnu"),
        password: String::from("knight"),
    };

    authenticate(cred);
}
