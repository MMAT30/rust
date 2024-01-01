fn main() {
   

   // string literals are slices
    let s = "Hello world!";


    // subtring of the first word
    let sub1 = first_word(&s[..]);
    println!("{}", sub1);



    // substring of a substring
    let sub2 = first_word(&s[1..]);
    println!("{}", sub2);

}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
