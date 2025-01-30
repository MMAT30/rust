fn main() {
    let v1 = false;
    let v2 = true;

    // if - else if - else
    if v2 {
        println!("if")
    } else if v1 && v2 {
        println!("else if")
    } else {
        println!("else")
    }


    // ternary operator
    let v3 = if v1 { "if" } else if v2 && v1 {"else if"} else { "else" };
    println!("{}", v3);

    // match
    let v4 = 4;
    match v4 {
        1 => println!("one"),
        2..3 => println!("two or three"),
        3 => println!("three"),
        4 => println!("four"),
        5..7 => println!("five or six or seven"),
        // use this only if you don't need anything to run in the default case
        _ => unreachable!(),
    }


    // loop
    let mut v5 = 0;
    loop {
        println!("{}", v5);
        v5 += 1;
        if v5 == 5 {
            break;
        }
        continue;
    }

    // while
    let mut v6 = 0;
    while v6 < 5 {
        println!("{}", v6);
        v6 += 1;
    }
}
