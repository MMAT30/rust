use std::collections::HashMap;
use std::fmt::Display;

pub trait Description {
    fn describe(&self) -> String;
}

pub trait Accomadation {

    // default implementation
    fn room(&self) -> String {
        "No rooms".to_string()
    }


    fn price(&self) -> f64;
}


pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}


impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Accomadation for Hotel<T> {
    fn room(&self) -> String {
        format!("{} has {} rooms", self.name, self.reservations.len())
    }

    fn price(&self) -> f64 {
        100.0
    }
}

impl<T: Display> Description for Hotel<T> {
    fn describe(&self) -> String {
        format!("{} is a hotel", self.name)
    }
}



#[allow(unused)]
pub struct AirBnB {
    name: String,
    reservations: HashMap<String, u32>,
}

impl AirBnB {
    pub fn new(name: &str) -> AirBnB {
        AirBnB {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
}

impl Accomadation for AirBnB {
    fn price(&self) -> f64 {
        50.0
    }
}

impl Description for AirBnB {
    fn describe(&self) -> String {
        format!("{} is an AirBnB", self.name)
    }
}


// function that takes a trait object
pub fn get_accomidation_price(accomidation: &impl Accomadation) -> f64 {
    accomidation.price()
}

// generic function that takes a trait object
pub fn get_accomidation_name<T: Accomadation>(accomidation: &T) -> String {
    accomidation.room()
}

// function that takes mutliple trait objects
pub fn get_accomidation_price_and_description(accomidation: &(impl Accomadation + Description)) -> f64 {
    accomidation.describe();
    accomidation.price()
    
}


// function that return a trait object
pub fn get_accomidation(accomidation: &impl Accomadation) -> &impl Accomadation {
    accomidation.price();
    accomidation
}
