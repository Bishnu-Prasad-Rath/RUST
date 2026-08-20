//Collections are the useful data structures
// The difference between collection and assemble data structure is that collection can carry multiple values.

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut vec = Vec::new();

    let mut vec1 = vec![1, 2, 3, 4, 5]; //We can also initial some default value as well by using '!' this macro .

    // let fourth_value = &v1[30]; //Here this line can crash the entire program so to prevent this make sure not to use it instead use the best practices for this kind of situations

    let fourth_value = v1.get(3).unwrap_or(-1); //It will return -1 instead of showing crashes

    println!("vec = {:?} and the 4rth value is : {}", vec1, fourth_value);

    for i in &mut vec1 {
        println!("i is {i}"); // We can also iterate on mutable as well by adding mut keyword.
        *i = *i * 2; //Here we are dereferencing the values means instead of multiplying the address with 2 we are multiplying the value that present in that addresss with 2.
    }
    let cells: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Text(String::from("Nice")), //In this way we can store mixed datatype in a vector.
        SpreadSheetCell::Float(2.0),
    ];

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
}
// In this case u must have to push same datatype of elements to the vector otherwise it will give u an error.
