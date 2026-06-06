fn main() {
    println!("Hello, world!");

    //In the concpet of keyword we can't use kw as a avariable because it is already reserved in RUST program.

    let age = 34;

     println!("{age}");

    //Here these are all about variables they can be changed by using let keyword and mut as well
    //In case of const u can't use mut keyword and it can't be mutate by itself.U have to type the variables with constant with uppercase letter.
    //In case of constant we have to all ensure that it must carry the type otherwise it will give errors types like  : u8,i32 etc.
    const PI:u8 = 10;

    println!("{PI}");
}
