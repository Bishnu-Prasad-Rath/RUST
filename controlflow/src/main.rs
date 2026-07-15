fn main() {
    println!("Hello, world!");

    let y: bool = true;

    // if y is true x should be 10 other wise x = 20

    // let mut x: i32 = 0;

    // if y{
    //     x = 10;
    // }else {
    //     x = 20;
    // }               We can write like this but there is a shortcut to write this that is : 

    let x : i32 = if y { 10 } else { 20 };   //Here it is mandatory to give an else because it act as an expression.One more thing that is if u return number type in if statement u must return same type in else statement as well if u write it.
                                             //We can say the code of if and else as arm.
    println!("The value of x is : {}", x);

    //Loop

    // loop{
    //     println!("This is an infinite loop");    //if we use the keyword loop then it can create an infinite loop for a certain task.
    // }

    let mut num: u8= 1;

    // loop{
    //     println!("Value of number is  : {}",num);
        
    //     if(num == 0){    //Here in this case the loop will break after 128 because 128 + 128 = 256 which is out of the range for u8 type and it can get back the value to 0 so if 0 happens the loop will break
    //         break;       //This block of code make the rust compiler panick.
    //     }

    //     num = num + num; //loops are also expressions
    // }

    let result ='my_loop : loop {
      println!("Value of number is {}",num);

    //   if num == 5{
    //     continue;
    //   }  

      if num == 10{
        break 70;      //We can also return a value after break 
      }

    //   loop{
    //     if num == 20 {
    //         // break;    //This break will work for the inner loop only.But if u want to break outer loop by adding break to inner loop then : 
    //         break 'my_loop 50;
    //     }
    //   }

      num = num+1;
    }; 

    println!("This is the end {}",result);

    //while loop

    let mut number = 3;

    while number !=0{
        println!("{}",number);
        number -= 1;
    }

    println!("LIFT_OFF!!!");

    let arr = [1,2,3,4,5,6];

    let mut index = 0;

    while index < 6 {
        println!("i : {} and v:{}",index,arr[index]);   //THis is not an optimized loop exectio because it everytime checks the bound condition whether it is in it's limit or not.
        index +=1;
    }

    for z in arr{
        println!("x = {}",x)
    }

    //There is an range feature from the standdard library we can use it for for loop

    for x in (1..=10).rev(){
        println!("x is {}",x);     //To get a reverse of range from 1 to 10 we can write the code like this by using for loop
    }


    }


