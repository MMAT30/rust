mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


// pathing convention
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
 
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


pub fn server_order() {}


pub mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // uses the parent module of the current module
        pub fn fix_incorrect_order() {
            super::server_order();
        }
}


pub fn eat_something() {

    // calling a public function from a public module
    back_of_house::fix_incorrect_order();

    let meal = back_of_house::Breakfast::summer("Rye");


    println!("I'd like {} toast please", meal.toast);
}


// this module is defined in another file called defined_lib.rs
pub mod defined_lib;
