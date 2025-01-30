use traits::{ Hotel, AirBnB, get_accomidation_price, get_accomidation_name, get_accomidation_price_and_description, get_accomidation, Description };


fn main() {
    

    let mut a1 = Hotel::new(String::from("String 1"));
    let mut a2 = AirBnB::new("AirBnB 1");


    println!("{} - {}", a1.room(), a1.price());
    println!("{} - {}", a2.room(), a2.price());


    println!("{}", get_accomidation_price(&a1));
    println!("{}", get_accomidation_price(&a2));


    println!("{}", get_accomidation_name(&a1));
    println!("{}", get_accomidation_name(&a2));


    println!("{}", get_accomidation_price_and_description(&a1));


    let a = get_accomidation(&a1);
    println!("{}", a.price());


    // variable implements the display trait that allows the describe method to be called
    a1.describe();


    // dynamic dispatch
    let d1: Vec<&mut dyn Description> = vec![&mut a1, &mut a2];
    for d in d1 {
        println!("{}", d.describe());
    }
}
