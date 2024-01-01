fn main() {
    let mut cnt = 0;

    // loop
    loop {
        cnt += 1;
        println!("looping...");

        if cnt == 3 {
            break;
        }
    }

    // returning values from loop
    cnt = 0;
    let data = loop {
        cnt += 1;
        println!("looping...");

        if cnt == 3 {
            break cnt * 2;
        }
    };

    println!("data is {}", data);

    // labeling loops - break at the loops label
    'loop1: loop {
        println!("loop1");
        break 'loop1;
    }
    
    

    // while loop
    cnt = 0;
    while cnt <= 3 {
        println!("{}", cnt);
        cnt += 1;
    }

    // for loop
    for x in 0..cnt {
        println!("{}", x);
    }
}
