fn main() {


    // vectors
    let mut v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // adding to the vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    println!("{:?}", v);

    // reading from the vector
    let third = v.get(2);
    match third {
        Some(value) => println!("{:?}", value),
        None => println!("No value found")
    }


    // removing from the vector
    let popped = v.pop();
    match popped {
        Some(value) => println!("{:?}", value),
        None => println!("No value found")
    }


    // slice from the vector
    let slice = &v[0..2];
    println!("{:?}", slice);


    // string vector
    let mut sv = vec![String::from("one"), String::from("two"), String::from("three")];
    for i in &sv {
        println!("{:?}", i);
    }


    // mutable string references
    let third = &mut sv[2];
    third.push_str(" & maybe four");

    for i in &sv {
        println!("{:?}", i);
    }



    // vector with capacity
    let mut v = Vec::with_capacity(2);
    println!("{:?}", v.capacity());
    v.push(1);
    println!("{:?}", v.capacity());
    v.push(2);
    println!("{:?}", v.capacity());
    v.push(3);
    println!("{:?}", v.capacity());
    v.push(4);
    println!("{:?}", v.capacity());
    
}
