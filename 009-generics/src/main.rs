use std::fmt::Display;

fn main() {


    // generic struct
    let p1 = Point { x: 12, y: 12 };
    let p2 = Point { x: 12.12, y: 12.12 };
    let p3 = Point { x: String::from("Hello"), y: String::from("World!") };

    p1.print();
    p2.print();
    p3.print();

    // generic enum
    let burger1: Burger<String> = Burger::Plain;
    let burger2 = Burger::Cheese(2);

    burger1.eat();
    burger2.eat();

}


// generic struct
struct Point<T> {
    x: T,
    y: T,
}


// generic implementation
impl<T: Display> Point<T> {
    fn print(&self) {
        println!("Point x: {}, y: {}", self.x, self.y);
    }
}

// generic enums
enum Burger<T> {
    Plain,
    Cheese(T)
}

impl<T: Display> Burger<T> {
    fn eat(&self) {
        match self {
            Burger::Plain => println!("Eating a plain burger"),
            Burger::Cheese(n) => println!("Eating a cheese burger with {} slices of cheese", n),
        }
    }
}


