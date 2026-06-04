fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;
    let z = 15;

    println!("The sum of {} and {} is {}", x, y, x + y);
    println!("The sum of {} and {} is {}", y, z, y + z);

    let mut a = 5;
    a += 10;
    println!("The value of a is {}", a);

    // In rust the variable life is in scope, so when the variable goes out of scope it will be dropped and the memory will be freed.
    {
        let b = 20;
        println!("The value of b is {}", b);
    }

    println!("The sum of 5 and 6 is {}", add(5, 6));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

//There are types of integer types that is signed interger and unsigned types, the signed integer types are i8, i16, i32, i64, i128 and isize, the unsigned integer types are u8, u16, u32, u64, u128 and usize. The difference between signed and unsigned integer types is that signed integer types can represent both positive and negative numbers while unsigned integer types can only represent positive numbers.
