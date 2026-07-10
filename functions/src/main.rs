fn main() {
    println!("Hello, world!"); //RUST compiler always finds the main function and runs it first, so we have to define the main function in our code.

    my_function(5); //calling the function and also we can call a function multiple times in the main function or any other function.
    is_even(4);
}

// snake casing is just a multiple word used when we define any function or variable or something similar divided by _

fn my_function(x: i32) {
    //We can take multiple input for a function with different data types but when we call a function we must pass same type of argument os it will not give an error
    //snake casing-->It also helps in code maintainability and readability
    println!("This is my function {}", x); //To call this function u have to pass an argument of type i32, and it will print the value of x passed to it.
}

//We can't run our own function by just defining it in RUST, we have to call it in the main function to run it.

// The curly braces part of an function is called the body of the function, and it contains the code that will be executed when the function is called.

//Each line of body of the function is called a statement, and each statement must end with a semicolon (;).

//Statements are instructiuons that perform some action, and they can be used to define variables, call functions, or perform other operations and do not return a value.

//Expressions evaluate to a resultant value.

//Calling a function is an expression because it evaluates to a value, and we can use the value returned by a function in other expressions or statements.

//In expressions we return a value but if we use ; then it will not return anything and shows errors.

fn add(x: i32, y: i32) -> i32 {
    //We can also return a value from a function, and we can specify the return type of the function using the -> operator followed by the type of the value being returned.
    x + y //This is an expression that evaluates to the sum of x and y, and it will be returned as the result of the function.
}

fn is_even(x: i32) -> bool {
    if (x % 2 == 0) {
        println!("The value is even");
        true
    } else {
        println!("The value is odd");
        false
    }
}
