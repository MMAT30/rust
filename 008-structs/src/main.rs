
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


trait Info {
    fn print(&self);
}

// methods
impl Info for User {
    fn print(&self) {
        println!("{}", self.username);
        println!("{}", self.email);
        println!("{}", self.sign_in_count);
        println!("{}", self.active);
    }
}

impl User {
    // acts as a constructor
    fn build_user(username: String, email: String) -> Self {
        Self {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    } 

    // immutable method, does not allow struct attributes to be changed but can be called
    fn get_username(&self) -> &String {
        &self.username
    }

    // mutable method, allows struct attributes to be changed
    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    // method that takes ownership of the struct
    fn move_user(self) -> Self {
        self
    }
}


// tuple structs
struct Color(i32, i32, i32);


// TODO: unit structs


fn main() {


    // creating a instance of a struct
    let u1 = User {
        username: String::from("Billy"),
        email: String::from("Billy@rust.com"),
        sign_in_count: 0,
        active: true,
    };
    println!("{}", u1.get_username());
    println!("{}", u1.email);
    println!("{}", u1.sign_in_count);
    println!("{}\n", u1.active);


    let u2: User = u1.move_user();
    print!("{}\n", u2.get_username());



    // creating a mutable instance based on another instance
    let mut u3: User = User {
        username: String::from("Billy Bob"),
        ..u2 
    };
    
    // using methods
    u3.print();

    // using 
    u3.set_username(String::from("Bob"));

    // print method
    u3.print();


    // creating a instance based on the methods attached to the struct
    let u4 = User::build_user(String::from("Greg"), String::from("Greg@rust.com"));
    u4.print();




    // tuple structs
    let black = Color(1, 2, 3);
    println!("\n{}", black.0);
    println!("{}", black.1);
    println!("{}", black.2);
}



