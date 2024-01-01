
#[derive(Debug)]
enum IpAddrKind {
    
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind
}



// the advatage of have a enum that holds diffrent types
// is because the enum type encompases all the types
// meaning the all enums are of type Message
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum States {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(States),
}

impl Message {
    fn call(&self){
        println!("call");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => {
            println!("Nickel");
            5
        },
        Coin::Dime => {
            println!("Dime");
            10
        },
        Coin::Quarter(s) => {
            println!("Quarter from {:?}",s);
            25
        },
    }
}

fn main() {

    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6("::1".to_string());

    println!("four: {:?}", four);
    println!("six: {:?}", six);



    // creating instance of struct
    let home = IpAddr {
        kind: IpAddrKind::V4(127,0,0,1),
    
    };

    println!("home: {:?}", home.kind);

    
    // 3 ways to create Option enum
    let n1: Option<i32> = Option::Some(5);
    let n2: Option<i32> = Some(5);
    let n3: Option<i32> = None;


    // we want to add n1 and n3
    // but we can't because n3 is None
    

    // this will crash the program at compile time
    // let sum: i32 = n1.unwrap() + n3.unwrap();

    // using a conditional unwrap
    let sum: i32 = n1.unwrap() + n3.unwrap_or(0);
    println!("sum: {}", sum);


    
    // using match
    let amt = value_in_cents(Coin::Penny);
    println!("amt: {}", amt);


    let amt = value_in_cents(Coin::Quarter(States::Alaska));
    println!("amt: {}", amt);

    
}




