fn main() {
    println!("Hello, world!");

    //In the concpet of keyword we can't use kw as a avariable because it is already reserved in RUST program.

    const AGE: u32 = 34;

    println!("{AGE}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3 + AGE;

    println!("THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS);

    //Here these are all about variables they can be changed by using let keyword and mut as well
    //In case of const u can't use mut keyword and it can't be mutate by itself.U have to type the variables with constant with uppercase letter.
    //In case of constant we have to all ensure that it must carry the type otherwise it will give errors types like  : u8,i32 etc.
    const PI: u8 = 10;

    println!("{PI}");

    let apples = 10;
    println!("{}", apples);
    //I know these are 2 values with same variables but the interresting thiung is these are not the same variable even thoushg the type the naming are same they are not same at all.
    let apples = apples + 10;
    println!("{}", apples);
    // Redelcaring the same named variable with same type with same name butwith different value is known as shadowing.
    //The 2nd apples is shadowed by the first apple.

    //Output guess

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of the x in the inner scope {}", x);
    }

    println!("The value of x is : {}", x);
}
