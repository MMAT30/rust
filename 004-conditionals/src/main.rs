fn main() {

    let x = 5;

    // if - else - else if
    if x == 5 {
        println!("x is 5");
    } else if x == 6 {
        println!("x is 6");
    } else {
        println!("x is not 5 or 6");
    }


    match x {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4 => println!("three"),
        5 => println!("four"),
        6..=10 => println!("six to ten"),
        _ => println!("something else"),
    }

    let bl: bool = match x {
        1 => true,
        5 => true,
        _ => false,
    };

    println!("bl is {}", bl);


    // ternary operator
    let y = if x == 5 { 10 } else { 15 };
    println!("y is {}", y);

}
