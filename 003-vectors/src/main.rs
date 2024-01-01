fn main() {

    // creating a new vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("{:?} len {}", v1, v1.len());


    // using a macro to create a vector and intialize it
    let mut v2 = vec![1, 2, 3];
    println!("{:?} len {}", v2, v2.len());

    // pushing a value onto a vector
    v2.push(4);
    println!("{:?} len {}", v2, v2.len());


    // indexing a vector
    println!("{}", v2[0]);
    println!("{}", v2[1]);
    println!("{}", v2[2]);
    println!("{}", v2[3]);

    // poping a value off a vector
    let popped = v2.pop();
    println!("{:?} len {} poped {}", v2, v2.len(), popped.unwrap());


    // iterating over a vector
    for i in v2.iter() {
        println!("{}", i);
    }

    // enumerating over a vector
    for (i, j) in v2.iter().enumerate() {
        println!("{} {}", i, j);
    }


    // iterating over a vector and changing the values
    for i in v2.iter_mut() {
        *i += 1;
    }
    println!("{:?}", v2);
}
