fn main() {

    let mut str = String::from("hello"); 
    str.push_str(" world!");   

    println!("{}", str);



    // x and y are copy of each other
    let x = 5;
    let y = x;

    let x_ptr = &x as *const i32;
    let y_ptr = &y as *const i32;

    println!("x = {} at {:?}", x, x_ptr);
    println!("y = {} at {:?}", y, y_ptr);


    // 
    let s1 = String::from("hello");
    let s1_ptr = &s1 as *const String;
    println!("s1 = {} at {:?}", s1, s1_ptr); 

    let s2 = s1;
    let s2_ptr = &s2 as *const String;
    println!("s2 = {} at {:?}", s2, s2_ptr); 

    // error: value borrowed here after move
    

    // s1 and s2 are not copy of each other, they both point to the same memory location
    



    // the example above would work if we use clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // creating pointers to s1 and s2
    let s1_ptr = &s1 as *const String;
    let s2_ptr = &s2 as *const String;

    println!("s1 = {} at {:?}", s1, s1_ptr);
    println!("s2 = {} at {:?}", s2, s2_ptr);


    // functions and ownership
    let s = String::from("hello");  
    takes_ownership(s);      
    // println!("{}", s); // error: value borrowed here after move       


    let x = 5;
    makes_copy(x);
    println!("{}", x); // this works because i32 is copy


    let s = gives_ownership();
    println!("{}", s);


    let s = takes_and_gives_back(s);
    println!("{}", s);




    // borrowing
    let mut s = String::from("borrowing");
    let len = calculate_lenght(&s);

    println!("The length of '{}' is {}.", s, len);



    let mut changed = change(&mut s);
    println!("{}", s);



    // slices
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("first word is {}", word);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String { 
    a_string
}

fn calculate_lenght(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" something");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}