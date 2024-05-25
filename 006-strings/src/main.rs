fn main() {
    

    // compiled into the binary
    let s1: &'static str = "Hello World!";
    println!("{}", s1);


    // length
    println!("{}", s1.len());


    // splitting by delimiter
    for word in s1.split_whitespace() {
        println!("{}", word);
    }

    // splitting by delimiter and reversing
    for word in s1.split_whitespace().rev() {
        println!("{}", word);
    }


    // vector to a string
    let mut v1: Vec<char> = s1.chars().collect();
    println!("{:?}", v1);


    // sorting a char vector
    v1.sort();
    println!("{:?}", v1);


    // creating a raw string that ignores escapes
    let raw_string = r"
        \x75\x6e\x69\x63\x6f\x64\x65
    ";
    println!("{}", raw_string);


    // creating a raw string that ignores quotes
    let raw_string = r#"
        "Hello
        World"
    "#;
    println!("{}", raw_string);



}
