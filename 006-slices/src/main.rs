fn main() {
    // string
    let str = String::from("I'm a Slice");
    println!("{}", str);

    // string slice
    let str_slice = &str[0..5];
    println!("{}", str_slice);

    // string literal
    let str_literal = "I'm a String Literal";
    println!("{}", str_literal);

    // string literal slice
    let str_literal_slice = &str_literal[0..5];
    println!("{}", str_literal_slice);

    // array slice
    let arr = [1, 2, 3, 4, 5]; 
    let arr_slice = &arr[0..3];
    println!("{:?}", arr_slice);

    // mutable slice
    let mut arr = [1, 2, 3, 4, 5];
    let arr_slice = &mut arr[0..3];
    arr_slice[0] = 10;

    println!("{:?}", arr_slice);
    println!("{:?}", arr);
    
}
