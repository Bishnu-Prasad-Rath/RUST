#[derive(Debug)]

enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, world!");

    let coin = Coin::Penny;

    println!("Value is {}",value_in_cent(&coin));

    println!("Add result = {}", add(50, Some(90)));

    let dice_roll = 4;

    //if no. is 3 then give it fancy_hat or if no. is 7 then remove the fancy hat 

    match dice_roll{

        3 => println!("You got a fancy hat."),
        6 => println!("You removed the fancy hat"),
        other => println!("Move {dice_roll} steps")  //Here u donn't need to handle all the values of matches if the cases are big and same type then u can handle with one line of code all at once.
    }
}


fn add(num : i32, num2 : Option<i32>) -> i32{
    match num2{
        Some(i) => num + i,      //This is known as patter matching
        None => num,                  //We have to handle all the cases otherwise it will give an error 
    }
}

fn value_in_cent(coin : &Coin)-> u8{
  match coin{
    Coin::Penny => {
        println!("This is a Penny");
        1
    },  //One line is known as arm and a match is kinda like swithc case but slightly different and we can make curly braces but if we write sub-code
    Coin::Nickel => 5,
    Coin::Dime => 20,
    Coin::Quarter(state)=>{
        println!("Got a Q of value {:?})",state);
        25
    },
    Coin::Quarter(UsState::Alaska)=>{
        println!("Hello Alaska");
        25
    },               //Here in match the order matters I mean in RUST the order of any case matters.
  }
}