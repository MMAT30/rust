use core::str;

fn main() {
    println!("Hello, world!");
    let a  = adder( 1, 2);
    println!("{}", a);

    let rptr = reapeter;

    rptr("Hello, world!");
}


fn adder(a: i32, b: i32) -> i32 {
    a + b
}

fn reapeter(str: &str) {
    
    println!("{}", str);
}
