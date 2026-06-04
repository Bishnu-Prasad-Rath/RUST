//Here I am creating a guessing game where user will guess the number in between 1 to 100 and system will generate the unmber randomly if user correct then success otherwise try again.

use rand::{Rng, RngExt};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng(); // No thread_rng()!
    let secret_number: i32 = rng.random_range(1..=100);

    // println!("Generated secret number:{}", secret_number);
    loop {                                           // loop is used to repeat the code until the user guesses the number correctlyz    
        println!("Guess a number");

        // prelude is a collection of items that are imported into every Rust program
        // by default, so we can use them without having to import them explicitly
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        let mut guess: i32 = match guess
            .trim()
            .parse() {
            Ok(num)=>num,
            Err(err)=>{
                println!("Please enter a valid number: {}", err);
                continue;
            }
        }; 

        match guess.cmp(&secret_number) {
            // Match is like switch ca se in Js but it has conditions for using it that is u must cover all possible cases otherwise it will give an error
            // The compare method as well takes teh reference of the secret_number not the ownsership of it.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
// semantic version of dependencies means MAJOR.MINOR.PATCH
// MAJOR version when you make incompatible API changes,
// MINOR version when you add functionality in a backwards-compatible manner, and
// PATCH version when you make backwards-compatible bug fixes.
