
// defining a basic enum
enum Colors {
    RED,
    YELLOW,
    BLUE,
}



// defining a basic enum that holds a enum attribute, which is not a struct
enum ColorType {
    RGB(i32, i32, i32)
}

#[allow(dead_code)]
enum Data {
    Map{i: i32, f: f32, s: String},
    Int(i32),
    Float(f32),
    Text(String),
}


impl Data {
    #[allow(dead_code)]
    fn print(&self) {
        match self {
            Data::Int(i) => {
                println!("enum type Int(i32)");
                println!("i = {}", i);
            },
            Data::Float(f) => {
                println!("enum type Float(f32)");
                println!("f = {}", f);
            },
            Data::Text(s) => {
                println!("enum type Text(String)");
                println!("s = {}", s);
            },
            Data::Map{i, f, s} => {
                println!("enum type Map{{i: i32, f: f32, s: String}}");
                println!("i = {}, f = {}, s = {}", i, f, s);
            },
        }
    }
}



impl ColorType {

    fn get_rgb(&self) -> (i32, i32, i32) {
        match self {
            ColorType::RGB(r, g, b) => {
                (*r, *g, *b)
            },
        }
    }

    fn print(&self) {
        match self {
            ColorType::RGB(r, g, b) => {
                println!("r = {}, g = {}, b = {}", r, g, b);
            },
        }
    }
}






// defining a basic enum that has methods attached to it
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// implementing a method on the enum
impl Coins {
    // the method take a reference to the enum
    fn in_cents(&self, coins: i32) -> f32 {
        let total = match self {
            Coins::Penny => {
                0.01 * coins as f32
            },
            Coins::Nickel => {
                0.05 * coins as f32
            },
            Coins::Dime => {
                0.10 * coins as f32
            },
            Coins::Quarter => {
                0.25 * coins as f32
            },
        };
        total 
    }
}

fn main() {
    
    // print the the underlying value of the enum
    println!("{}", Colors::RED as i32);
    println!("{}", Colors::YELLOW as i32);
    println!("{}\n", Colors::BLUE as i32);


    // print the the underlying value of the enum
    let color = ColorType::RGB(255, 0, 0);

    // calling methods attached to the enum
    println!("{:?}", color.get_rgb());
    color.print();
    

    // defining coin types
    let p = Coins::Penny;
    let n = Coins::Nickel;
    let d = Coins::Dime;
    let q = Coins::Quarter;


    // print the coin types with the attached methods
    println!("{}", p.in_cents(3));
    println!("{}", n.in_cents(3));
    println!("{}", d.in_cents(3));
    println!("{}", q.in_cents(3));

    // TODO: migrate this code out
    let option = Some(5);
    let option2 = Some("Hello");
    let option3 = None;

    println!("{}", option.unwrap());
    println!("{}", option2.unwrap());
    println!("{}", option3.unwrap_or(0));


    
}


