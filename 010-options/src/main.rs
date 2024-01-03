




#[derive(Debug)]
enum States {
    Alaska
}

enum Coin {
    Penny,

    Quarter(States),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Quarter(s) => {
            println!("Quarter from {:?}",s);
            25
        }
    }
}

fn main() {

    
    
    // 3 ways to create Option enum
    let n1: Option<i32> = Option::Some(5);
    let n2: Option<i32> = Some(5);
    let n3: Option<i32> = None;

    println!("n1: {:?}", n1);
    println!("n2: {:?}", n2);
    println!("n3: {:?}\n", n3);


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


    // TODO: example of if-let

    
}




