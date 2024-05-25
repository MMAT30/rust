fn main() {

    // x and y are copy of each other
    // there two variable point to the same place in memory
    let x = 5;
    let y = x;

    println!("x = {}", x);
    println!("y = {}\n", y);

    // addresses are the same
    println!("x = {:p}", &x);
    println!("y = {:p}\n", &y);





    // strings are different
    // s1 is the owner of the memory location
    // s1 and s2  point to the same memory location
    // but s2 is the owner of the memory location
    let s1 = String::from("hello");
    let s2 = s1;

    // since s2 is the owner of the memory location, this will fail
    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    
    
   
    
    // the only way to make s3 and s4 both have the same value is to clone
    // s3 and s4 are different memory locations with the same value
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}", s3);
    println!("s4 = {}", s4);


    // this function give ownership to the function
    // but never give it back thus making the variable s4 invalid
    let s4 = String::from("hello");  
    takes_ownership(s4);      
    // println!("{}", s4); // error: value borrowed here after move       


    // integers work differently
    // this function makes a copy of the value
    // thus making the variable w still valid
    let w = 5;
    makes_copy(w);
    println!("{}", w); // this works because i32 is copy


    // creating a string inside the scope of the function
    // and returns it x variable
    let x = gives_ownership();
    println!("{}", x);



    // takes a string and returns it
    // this function takes ownership of the string but reassigment is necessary
    let y = takes_and_gives_back(x);
    println!("{}", y);




    // borrowing the value of a string and not munipulating it
    let z = "borrowing";
    let len = calculate_lenght(&z);

    println!("The length of '{}' is {}.", z, len);



    // mutable references that can be changed
    let mut before = String::from("hello");
    change(&mut before);
    println!("{}", before);

}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);

} 

fn takes_and_gives_back(a_string: String) -> String { 
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn calculate_lenght(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" something");
}

