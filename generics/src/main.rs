fn main() {
    
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    let v3 = vec!['a', 'b', 'c', 'd', 'e'];

    // generic function
    let r1 = get_largest(v1);
    let r2 = get_largest(v2);
    let r3 = get_largest(v3);

    println!("The largest number is {}", r1);
    println!("The largest number is {}", r2);
    println!("The largest number is {}", r3);



    // generic structs
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = Point { x: 5, y: 10.4 };

    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    // using the generic method
    let p4 = p1.mix_up(p2);
    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);



    // generic enums
    let o1 = Opt::Some(5);
    let o2 = Opt::Some("Hello");
    let o3: Opt<i32> = Opt::None;


    println!("o1 wrapped = {:?}", o1);
    println!("o2 wrapped = {:?}", o2);
    println!("o3 wrapped = {:?}", o3);

    // generic methods of enums
    println!("o1 unwrapped = {}", o1.unwrap());
    println!("o2 unwrapped = {}", o2.unwrap());

    // this will panic, because we are trying to unwrap a None value
    // println!("o3 unwrapped = {}", o3.unwrap());

    
}



// <T> denotes a generic type and we return a generic type
// <T: PartialOrd + Copy> denotes that T must implement the PartialOrd and Copy traits
fn get_largest<T: PartialOrd + Copy>(v: Vec<T>) -> T {
    let mut largest = v[0];

    for i in v {
        if i > largest {
            largest = i;
        }
    }

    largest
}


struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            // original x = T
            x: self.x,

            // other.y = W
            y: other.y,
        }
    }
}


#[derive(Debug)]
enum Opt<T> {
    Some(T),
    None,
}


impl<T> Opt<T> {
    fn unwrap(self) -> T {
        match self {
            Opt::Some(i) => i,
            Opt::None => panic!("Called unwrap on a None value"),
        }
    }
}   

