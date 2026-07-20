fn main() {
    //Reference is a pointer that points to a valid type value
    println!("Hello, world!");

    let mut s= String::from("Bishnu Prasad Rath");
    let len = calculate_len(&mut s);

    println!("The len of {s} is {len}");  //There is a catch that if u have a mutable reference value then u can't reference 2nd time to that specific value 
    //Otherwise it will show an error if u are trying t mutate the value during multiple references.

    let mut k = String::from("Hello");

    let r1 = &k;
    let r2 = &k;

    println!("{r1} and {r2}");

    let r3 = &mut s;  //Here it will not give u an error becuase the reference span of r1 and r2 is lastly used on line number
    //16 after that u can create a mutable reference of the same value.s

    println!("{r3}");


    //Dangling references
    //Dangling Pointer : A pointer that references a location in memory that may have been given to someone else by feeing some memory while preserving a pointer to that memory.

    // let reference_to_nothing = dangle();


}

//To referencing anytyhing we have to use & and for de-referencing we have to use *
//If there is are imutable references are there u can't again create a mutable reference of the same value otherwise it will give u an error.
// The life sapn of a references is depending on the line which is lastly used.
fn calculate_len(s: &mut String) ->usize{
    s.push_str(" Namaste");
    let result = s.len();
    result
}


// fn dangle()->&String{
//     let s = String::from("Kn44");
//     &s
// }    //Here in this case the life of s is in that dangle function and if it dies in that function then how reference will work because 
//the memory is already freed and the &s is pointing to a freed memory so it will give u an error and this is known as Dangling references.
