fn main() {
    println!("Hello, world!");
    //RUST is know for his memory management by work with ownerhip.
    //If your code can't manage the memory perfectly in RUST it will give u the compilation error.
    
    //In RUST there are also concepts available known as heap and stack.
    //Whenever a data size is known and fixed then we use stack and if any data is not fixed in size or unknown then we use heap.
    let s = "hello world";  //This value is valid until it belongs to this particular scope and it's owner is main function.
    {
        let x = "This is another value";
        //X value starts in this scope and it ends in this scope as well not in main scope.

        //There is new datatype which have unknown size and it is now playing with heap that is known as String type.
        //To understand ownership we have to use string.
        //String is a dynamic allocated data.
    }

    let mut k = String::from("This is a string value");  //It can be growable..
    k.push_str(".Hello kn44");  //This is the requested part to allocate the memore for this string and it is temporary after running the program it will give it back the temp memory
    println!("K = {}",k);
    
}
