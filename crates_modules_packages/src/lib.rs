#![allow(dead_code, unused_variables)] //All functions are private to other file but to make it public then we have to add pub 

pub struct Credentials {
    //If any function is related to other function then we have to also add pub to that function as well.
    pub username: String,
    pub password: String,
} //In a struct if u want to access required attribute of the struct then u have to add pub to all of them as well for accessing those attributes.

mod database;

pub mod auth_utils;

use auth_utils::login;
use database::{Status, connect_to_database};

pub fn authenticate(cred: auth_utils::models::New) {
    if let Status::Connected = connect_to_database() {
        login(cred);
    }
}

pub mod util;
//Here this line will see 2 ways of execution of this utils one is it will inspect to src/util.rs or src/util/mod.rs
//To use it on main.rs we have to add pub keyword as well and in the utils.rs we have also add the pub keyword.
//src/utils.rs  --> THis is mode prefered style for any RUST project
//src/util.mod.rs
//This is the clean version of RUST project it is well organized str.
