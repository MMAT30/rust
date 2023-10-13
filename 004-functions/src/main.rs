fn main() {

    // function call
    println!("{}", adder(5, 5));


    // variable binding to an expression
    let x = {
        let y = 10;
        y + 5
    };        
    println!("{x}"); 



    let (one, two) = nums(1, 2);
    println!("one = {}, two = {}", one, two);

}

fn adder(x: i32, y: i32) -> i32 {
    x + y
}

fn nums(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}


