use std::mem;

fn main() {

    // creating heap data
    let v1 = String::from("one");
    let v2 = String::from("two");

    println!("v1 {}", v1);
    println!("v2 {}", v2);
    println!("v1 {:p}", &v1);
    println!("v2 {:p}", &v2);
    println!("v1 {}", mem::size_of_val(&v1));
    println!("v2 {}\n", mem::size_of_val(&v2));


    // cloning heap data
    let v3 = v1.clone();


    println!("v1 {}", v1);
    println!("v3 {}", v3);
    println!("v1 {:p}", &v1);
    println!("v3 {:p}", &v3);
    println!("v1 {}", mem::size_of_val(&v1));
    println!("v3 {}\n", mem::size_of_val(&v3));


    // reference to heap data
    let v4 = &2;
    println!("v4 {}", v4);
    println!("v2 {}", v2);
    println!("v4 {:p}", &v4);
    println!("v2 {:p}", &v2);
    println!("v4 {}", mem::size_of_val(&v4));
    println!("v4 {}\n", mem::size_of_val(&v4));

    //reference to stack data
    let v5 = 2;
    let v6 = &v5;
    println!("v5 {}", v5);
    println!("v6 {}", v6);
    println!("v5 {:p}", &v5);
    println!("v6 {:p}", v6);
    println!("v5 {}", mem::size_of_val(&v5));
    println!("v6 {}\n", mem::size_of_val(&v6));


    // cloning heap data
    let v7 = String::from("seven");
    print_value(v7.clone());
    println!("v7 {}\n", v7);

    
    // mutating heap data in place with mutable reference
    let mut v8 = String::from("eight");
    add_to_string(&mut v8);
    println!("v8 {}", v8);


    // function with return value
    let v9 = sum(1, 2);
    println!("v9 {}", v9);

}


fn print_value(value: String) {
    println!("{}", value);
}

fn add_to_string(value: &mut String) {
    value.push_str(", world!");
    println!("{}", value);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}