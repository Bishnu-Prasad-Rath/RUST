mod front_of_house {
    mod hosting {

        fn add_to_wishlist() {}

        fn seat_at_table() {}
    }

    mod serving {

        fn take_order() {}

        fn server_order() {
            crate::fron_of_house::hosting::seat_at_table(); //This is absolute path
            super::hosting::add_to_wishlist(); // This is relateive path
        }

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use back_of_house::Breakfast;
use front_of_house::hosting;

fn eat_at_restaurant() {
    let b1 = Breakfast::summer("Wheat");
    hosting::add_to_wishlist();
}

//There are two way or path to define any odule that is absolute path and relative path
//Here absolute path means starts from the very start point that is from the crate. It uses crate keword
//Relative path means start from the current path and it uses super keyword.
//If by any chance ur path is not working then it can be a possibility that the module u want to use is not public.
//In many cases we can access a path through relative path but not possible with absolute path.
//The use keyword can't be used on nested modules.
