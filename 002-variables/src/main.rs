// constants
const NUMBER: i8 = 5;
// type alias
type Meters = i8;

fn main() {


    // non-mutable variable
    let x = 5_000_000;
    println!("The value of x is: {}", x);

    // this will cause an error
    // x = 6;

    // mutable variable
    let mut y = 5;
    println!("The value of y is: {}", y);

    y = 6;
    println!("The value of y is: {}", y);


    // constants
    println!("The value of NUMBER is: {}", NUMBER);

    // type alias
    let distance: Meters = 5;
    println!("The value of distance is: {}", distance);

    // character
    let c: char = 'a';
    println!("{}", c);

    // string
    let s: &str = "C:\\bin\\bash";
    println!("{}", s);

    // float format specifier
    let f: f32 = 3.14123456789;
    println!("{:.2}", f);

    // casting float to int
    let f: f32 = 3.14123456789;
    let i: i32 = f as i32;
    println!("{}", i);

    // boolean
    let b1: bool = true;
    let b2: bool = false;
    println!("{}", !b1);
    println!("{}", b1 && b2);
    println!("{}", b1 || b2);

    // array
    let mut arr = ["one", "two", "three", "four", "five"];

    arr[0] = "one hundred";
    
    for i in arr.iter() {
        println!("{}", i);
    }


    // tuple
    let tup  = ("bob", 25, 5.11);
    println!("{} is {} years old and {} feet tall", tup.0, tup.1, tup.2);


    // range
    let month_days = 'a'..='z';
    for i in month_days {
        println!("{}", i);
    }

}
