use std::env;
use std::io;

fn main() {

    // command line arguments
    let args: Vec<String> = env::args().collect();
    
    for arg in args.iter() {
        println!("{}", arg);
    }


    println!("Enter a string:");

    // input from stdin
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input);
}
