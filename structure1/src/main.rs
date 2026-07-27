// Write a program to calculate the area of a rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 32,
        height: 50,
    };

    let area = calculate_area(&rect);

    // dbg!(rect);  //U can use the dbg! here to print directly rect but it will take the ownership of react so u can use it depend on the usecase
    // And the difference between dbg! and println! is that println! is not taking any ownership but dbg! will take the ownerhip and it also give the info like in whcih line the struct loses it's ownership.
    //dbg! is very usefule when we write or print the rect with reference means it will not take the ownership of react
    //dbg! also returns some values as well.

    dbg!(&rect);

    println!("The area of the Reactangle {:#?} is {}", rect, area)
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// In this code the function and the strcut is not tied up with each other to do this we have to use Method Syntax.
