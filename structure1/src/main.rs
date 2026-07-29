// Write a program to calculate the area of a rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn claculate_area(&self,e : u32)->u32 {
        self.height * self.width
    }

    fn can_hold(&self, other : &Rectangle)->bool{
       self.width >= other.width && self.height >= other.height
    }

    fn square(side : u32)->Rectangle{
      Rectangle{                        // Here we can implement or make manipulate the rectangle to square as well
        width : side,
        height : side,
      }
    }
}

fn main() {
    let rect = Rectangle {     //In this object the rect is pointing to the &self in the calculate_area funciton when we call the function
        width: 32,                       
        height: 50,
    };

    let rect2 = Rectangle {
        width : 5,
        height : 40,
    };

    let sq = Rectangle::square(5);

    println!("Can rect1 hold rect2 {}",rect.can_hold(&rect2));

    rect2.claculate_area(2);   //We can also pass extra argments if needed
    
    // let area = calculate_area(&rect);

    // dbg!(rect);  //U can use the dbg! here to print directly rect but it will take the ownership of react so u can use it depend on the usecase
    // And the difference between dbg! and println! is that println! is not taking any ownership but dbg! will take the ownerhip and it also give the info like in whcih line the struct loses it's ownership.
    //dbg! is very usefule when we write or print the rect with reference means it will not take the ownership of react
    //dbg! also returns some values as well.

    dbg!(&rect);

    println!("The area of the Reactangle {:#?} is {}", rect, rect.claculate_area(2))

    //String::from is a associated function that is define on the string
}

// fn calculate_area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// In this code the function and the strcut is not tied up with each other to do this we have to use Method Syntax.

// So the concept is if the function is declared in the context of struct then it is known as method.The first parameter is self.
