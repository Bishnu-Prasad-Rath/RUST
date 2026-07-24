struct User {
    active: bool,
    username: String, //Owner type so it can take the ownership
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user_1 = User {
        email: String::from("Bishnu Prasad Rath"),
        username: String::from("Bishnu Prasad Rath"),
        active: true,
        sign_in_count: 0,
    };

    user_1.username = String::from("something_else");

    println!("Changed user_1 name is : {}", user_1.username);

    let user_2 = User {
        email: String::from("Bishnu Prasad Rath"),
        username: String::from("Bishnu Prasad Rath"),
        active: true,
        sign_in_count: 0,
    };

    println!("The name of the user_2 is : {}", user_2.username);

    let user_3 = build_user(
        String::from("Bishnu Prasad Rath"),
        String::from("rbishnu604@gmail.com"),
    );

    // To oup date any user for example if we make the ownership of user_2 to user_4 then we have to update the attrigbute and then write like this

    let user_4 = User {
        email: String::from("Bishnu55707x"),
        active: false,
        ..user_2
    };

    //  println!("user_2 username is : {}",user_2.username);   //We can't write this code bcause the ownership is moved from user_2 to user_4

    println!("Updated user is {}", user_4.username);

    let red = (100, 0, 0);
    set_bg_color(red);

    let point = (30,40,90);
    move_point(point);
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0,
    }
}

//RGB
fn set_bg_color(color: (u8, u8, u8)) {
    println!(
        "Setting background color R={},G={},B={}",
        color.0, color.1, color.2
    )
}

fn move_point(point:(u8,u8,u8)){
    println!("The cursor was moved Y={},X={},Z={}",point.0,point.1,point.2);
}