fn main() {

    // creating struct instance
    let mut p1 = Person {
        name: String::from("Alice"),
        age: 30,
        introduced: false
    };

    // modifying struct fields
    p1.name = String::from("Bob");
    p1.age = 40;

    // accessing struct fields
    print_person(&mut p1);


    // creating struct from a function
    let mut p2 = create_person(String::from("Alice"), 30, false);
    print_person(&mut p2);

    // creating struct from a struct
    let mut p3 = Person {
        name: String::from("Alice"),
        ..p2
    };

    print_person(&mut p3);
    introduce(&mut p3);

    // using debug trait
    println!("{:?}\n", p3);

    // using methods
    let mut p4 = Person::new(String::from("Joe"), 77, false);
    p4.print_person();
    p4.introduce();

    // using method with parameters
    let is_older = p4.older_than(&p3);
    println!("Is p4 older than p3? {}\n", is_older);


    // using tuple struct
    let c1 = Color(255, 0, 0);
    println!("c1.0: {}", c1.0);
    println!("c1.1: {}", c1.1);
    println!("c1.2: {}\n", c1.2);


    // using unit struct
    let u1 = UnitStruct;
    println!("{:?}\n", u1);
}




// creating struct from a function
fn create_person(name: String, age: u8, introduced: bool) -> Person {
    Person {
        name,
        age,
        introduced
    }
}


// function that takes in a struct
fn print_person(p: &mut Person) {
    println!("Name: {}", p.name);
    println!("Age: {}", p.age);
    println!("Introduced: {}\n", p.introduced);
}

fn introduce(p: &mut Person) {
    p.introduced = true;
    print_person(p);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    introduced: bool
}

impl Person {
    fn new(name: String, age: u8, introduced: bool) -> Person {
        Person {
            name,
            age,
            introduced
        }
    }

    fn introduce(&mut self) {
        self.introduced = true;
        self.print_person();
    }

    fn print_person(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Introduced: {}\n", self.introduced);
    }

    fn older_than(&self, other: &Self) -> bool {
        self.age > other.age
    }
}


// tuple struct
struct Color(u8, u8, u8);

// unit struct
#[derive(Debug)]
struct UnitStruct;
