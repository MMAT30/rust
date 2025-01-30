use std::collections::HashMap;

fn main() {

    // hashmap
    let mut h1: HashMap<String, f64> = HashMap::new();


    // inserting values
    h1.insert(String::from("one"), 1.0);
    h1.insert(String::from("two"), 2.0);
    h1.insert(String::from("three"), 3.0);
    println!("{:?}", h1);


    // array to hashmap
    let a1: [(String, f64); 3] = [
        (String::from("one"), 1.0),
        (String::from("two"), 2.0),
        (String::from("three"), 3.0),
    ];


    // converting array to hashmap
    let h2 = HashMap::from(a1);
    println!("{:?}", h2);

    // removing values
    h1.remove(&String::from("two"));
    println!("{:?}", h1);


    // creating variables that hold owenership
    let mut h3: HashMap<&str, f64> = HashMap::new();
    let o1 =  String::from("one");


    // passing ownership to hashmap
    h3.insert(&o1, 1.0);
    println!("{:?}", h3);


    // accessing the hashmap
    let v1 = h3.get("one").unwrap_or(&123.123);
    println!("{:?}", v1);


    // overwriting values
    h3.insert("one", 100.0);
    println!("{:?}", h3);


    // entry - or insert
    let v2 = h3.entry("two").or_insert(2.0);
    println!("{:?}", v2);

    // remove
    h3.remove("two");
    println!("{:?}", h3);


    // hash set
    let mut hs1: HashMap<&str, f64> = HashMap::new();
    hs1.insert("one", 1.0);
    hs1.insert("two", 2.0);
    hs1.insert("three", 3.0);
    println!("{:?}", hs1);

    // inserting the same values
    hs1.insert("one", 1.0);
    hs1.insert("two", 2.0);
    hs1.insert("three", 3.0);
    println!("{:?}", hs1);


}
