use std::collections::HashMap;


fn main() {
  

    // creating a new hash map
    let mut scores: HashMap<String, i8> = HashMap::new();


    // inserting values into the hash map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 100);

    println!("{:?}", scores);


    // getting the entry enum
    let r1 = scores.entry(String::from("Green"));
    println!("{:?}", r1);


    // the or insert method will insert a value if the key does not exist
    r1.or_insert(25);
    println!("{:?}", scores);




    
    // getting a certain value
    let r2 = scores.get(&String::from("Blue"));

    match r2 {
        Some(value) => println!("The value is {}", value),
        None => println!("No value found"),
    }

    // another way to get a value
    println!("{}", r2.unwrap());




    
}
