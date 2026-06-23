fn main() {
    // there are 2 kind of datatypes in RUST that is one is scalar and the other one is compound.

    let mut a = 1;

    a = 10;

    //  a = "10";  // In RUST the datatypes are static u can't change randomly datatype of a partivular variable.

    //The compiler will automatically infer the datatype based on developer value I mean what kind of value the developer is givintg to the variable.

    //The process of assinging the datatypes to a variable is known as annotation.

    // There is some certaijn conditions where u haev to explicitly add the datatypes because sometimes get confused because of it so if u see any kind problem then u have to add teh datatype explicitly.

    //Types of datatypes in RUST :

    //Scalar Types --> Single Type --> integer,floating-point numbers,decimal numbers

    //Interger is a type which is a number with no fractional component. Ex : u32 (unsigned integers) , i32 (signed integers)

    //u32 mean it is unsigned and takes 32 bits of space and same for other cases like i32 as well

    //There is a thing that is arch with isize and usize it will categorize the number of buits based on teh device u are coding right now.

    //The difference between signed and unsigned is that in unsigned negative numbers or less than 0 is not allowed but in signed datatypes it is allowed to keep a negative value as well positive value
    
    println!("Hello, world!");

}
