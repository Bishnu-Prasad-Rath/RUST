fn main() {
    // there are 2 kind of datatypes in RUST that is one is scalar and the other one is compound.

    //In scalar we can store single value on ther  other hand in compiund we can store multiple value in one variable.

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

    //Signed numbers are stored in 2S representation

    //Signed numbers can store -(2^n-1) to 2^n-1-1 where n is the number bitsthat variants uses.

    //Ex : i8 ranges from -128 to 127 if we exceed the limits then it will show out of range.

    //Unsigned can store from 0 to 2^n-1 range

    //Ex : u8 reanges from 0 to 255

    let mut c = 129;

    println!("Hello, world!");

    //There is another trick u can read a numeric value I mean if there are lot of zero in that number and it's hard to read how much zeo in it

    let k = 100_00_00_0; //This is a valid numeric value and this is easy to read u can write like this as well.

    println!("Value of k is {}", k);

    //Integer Overflow : if the range of the variable exceeds then it can cause integer overflow.
    //In production build like if u command cargo run --release then it will ignore this integer overflow issue, it wraps it up with 2's complement
    //The plus point is in production level the systejm will not crash but it can cause a hidden bug which can't be  found normally.

    //Floating point types.

    let v = 3.1; //f64 has more precisioin then f32

    let n: f32 = 3.0; //f32 has less precision

    //Example for the precision diffrence between f64 and f32

    let my_f64 = 2.123456789123456789;
    let my_f32: f32 = 2.123456789123456789;

    println!("my_f64 : {}", my_f64);
    println!("my_f32 : {}", my_f32);

    //There is an integer type behaviours like when an output result is in float type but the variable type is like integer type,
    //the number of the output will be rounded to integer type not in float type

    let x: i32 = 5 / 2; //logically the output will be 2.5 which is float type but it will be int type

    println!("X is {}", x); // 2 instead of 2.5

    //To make it float type u can write like this

    let z: f32 = 5_f32 / 2_f32; //2.5

    println!("Z is {}", z);

    //Character Type :
    let k = 'p'; // In character type it can contain only 1 character otherwise it will give an error.
    let s = "Bishnu"; //In string type it can contain multiple characters.

    //Tuple Types

    //It is generally one declared, they can not grow or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1); //Tuple is compound variable

    let bishnu = (21, true, 100);

    let (x, y, z) = bishnu;

    println!("0th value is {}", bishnu.0);
    println!("1st value is {}", bishnu.1);
    println!("2nd value is {}", bishnu.2);

    //when we create an tuple without any value then it is known as unit.

    let queue = (); // It is a Unit

    println!("x is {x}");
    println!("x is {y}");
    println!("x is {z}");

    //Array type

    //In an array type there is always same type will be maintained otherwise it will give an error.

    let array = [1, 2, 3, 4, 5];

    let array1 = [10; 5]; //This line of code means make a length of array 5 and in each index put the value 10.

    //If we print a value that is not present in an array then at compile time the system will panick and give an error like fore example :
    // println!(array[6]);
}
