fn main() {
    //Slice is a kind of reference which does not have ownership. It is a view into a block of memory represented as a pointer and a length. Slices can be used to borrow a section of an array or a vector without taking ownership of the entire collection.

    let mut s = String::from("Hello, world!");

    let res = find_first_word(&s);

    // s.clear(); //This line is a problem here becuase if we change the result and clear the string of s value then it will show a wroung output I mean if u run it.
    //To simply defend this problem we use Slice.

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("Hello : {hello}");
    // println!("World : {world}");

    println!("Result of {s} is {}", res);

    s.clear();

}

fn find_first_word(input: &str) -> &str {
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}
