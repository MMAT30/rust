fn main() {
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");

    // lifitime is resolved since we tell the compiler
    // that the return value of longest will live as long as
    // the smaller of the two arguments
    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);

    //
    let s3 = String::from("long string is long");
    {
        let s4 = String::from("xyz");
        let result = longest(s3.as_str(), s4.as_str());
        println!("The longest string is {}", result);
    }

    
    // this is a static lifetime
    // it lives for the entire duration of the program
    // and is stored in the read only memory of the binary
    // even if the function returns, the value will still be there
    let s: &'static str = "I have a static lifetime = forever";
 

    //
    println!("{}", s);
}

// this is a interesting case
// we know the function accepts two addresses of string
// but we dont know the exact address to return in each case
// do we return the address of x or y?
// the compiler doesnt know either
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
