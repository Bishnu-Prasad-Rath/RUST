fn main() {
    println!("Hello, world!");
    //RUST is know for his memory management by work with ownerhip.
    //If your code can't manage the memory perfectly in RUST it will give u the compilation error.

    //In RUST there are also concepts available known as heap and stack.
    //Whenever a data size is known and fixed then we use stack and if any data is not fixed in size or unknown then we use heap.
    let s = "hello world"; //This value is valid until it belongs to this particular scope and it's owner is main function.
    {
        let x = "This is another value";
        //X value starts in this scope and it ends in this scope as well not in main scope.

        //There is new datatype which have unknown size and it is now playing with heap that is known as String type.
        //To understand ownership we have to use string.
        //String is a dynamic allocated data.
    }

    let mut k = String::from("This is a string value"); //It can be growable..
    k.push_str(".Hello kn44"); //This is the requested part to allocate the memore for this string and it is temporary after running the program it will give it back the temp memory
    println!("K = {}", k);
    //The space will be free depends on the owner when th value goes out of scope it will automaticaclly clear the memory to save the space

    //Rust internally calls a function known as drop to free the memory when the value goes out of scope.

    let mut x = 5;
    let y = x; //This is copy of x stored in y 

    x = 20;

    println!("X is {x} and y is {y}");

    let b = String::from("I am X");

    // let mut c :String = b;   //In this copying the data to c whenever we call b it will give u error because b will be invald becuase c is referencing and this removes the problem of double free memory bug
    //So technicall y we moved the ownership of b to c by giving the whole b to c.
    //If u want to copy the value of the b to c then use b.clone()
    let c = b.clone(); //It is a expensive operation that is copying the whole data to heap memory.
    println!("c is {c}");
    //We can't copy data to heap because it's very expensive and it takes time as well because in case of string it is not actually copying the
    //actual value of the stgring it copies the metadata like ptr,len and capacity so it can access the actual string value.
    //If we assign the same memory to deifferent variable there can be issue known as double free memory error where if the memory is cleared for the first pointer and then with 2nd time cleared for 2nd pointer then it will create a problem

    //Function and ownership

    let num = 10;

    let result = add(num);

    println!("Num is {num} and result = {}",result);

    let name = String::from ("Bishnu Prasad Rath");

    takes_ownership(name);

    // println!("Value of the name is {name}"); //Here if we call name for any use case then it will give u an error beacucse the ownership is controlled by the s named variable present in that function
    //So if u 2nd time use this name then it will give u an error.

    let s = gives_ownership(); //Here we are getting the ownerhip of gives_ownership to s variable in the main function.

    let s2 = takes_and_gives_back(s);

    let s3  = String::from("Bishnu Prasad Rath");

    let (s4,length) = calculate_len(s3);

    println!("The length of {s4} is {length}")

}

fn takes_ownership(s:String){
    println!("Inside ownership {}",s);
}

fn gives_ownership()->String{
 let s = String::from("This is a string from give ownerships.");
 s
}

fn takes_and_gives_back(s:String)->String{
    println!("S in takes_and_gives_back {}",s);
    s
}

fn add(x:i32) -> i32 {
    x+10
}


fn calculate_len(s:String)-> (String,usize){
    let result = s.len();
    (s,result)
}