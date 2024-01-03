use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;
use std::error::Error;



// propagating errors
fn read_file() -> Result<String, io::Error> {

    // the (?) operator can be used to
    // return the catch the error and return it
    // panic is not called rather returned


    // create a string
    // let mut s = String::new();

    // // open file and read file
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // // converting the string to a result ok enum type since this what is expected
    // Ok(s)


    // TODO: fix this scoping issue
    fs::read_to_string("hello.txt")
}


fn main() -> Result<(), Box<dyn Error>> {

    // this will crash the program and start the backtrace
    // panic!("crash and burn");



    // what if don't want to crash the program?
    // we want the program to continue running 
    // without panicking, use Result<T, E>


    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }



    // we know that is file doesn't exist
    // so how do we catch it?
    // let file = File::open("hello.txt");


    // we can use match expression to handle the Result
    // if we do this it will crash if we mathch the error
    // we want to handle the error in a better way
    // we can use the std::io::ErrorKind enum to accomplish this



    // try this out, it panics because the file doesn't exist
    // copy the error kind because we will use it later
    // hint: { code: 2, kind: NotFound, message: "No such file or directory" }

    // let result = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };


    // better way to handle the error
    // if the file doesn't exist, create it
    // but there is a even better way to do this
    // try this out


    // let result = match file {

    //     // checking if the file exists or not
    //     Ok(file) => file,

    //     // if the file doesn't exist catch the error and create it
    //     Err(error) => match error.kind() {

    //         // checking if the file doesn't exist and creating it
    //         ErrorKind::NotFound => match File::create("hello.txt") {

    //             // if the file is created, return the file
    //             Ok(fc) => fc,

    //             // else there is not more we can do, panic 
    //             // we dont want need the panic here
    //             // but we are just showcasing the use of Result<T, E> and panic!
    //             Err(error) => {
    //                 panic!("Problem creating the file: {:?}", error)
    //             },
    //         },

    //         // we are also convering the other infinite errors by catching them
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         },
    //     },
    // };


    // there is a better way to do this
    // we can use the unwrap method to handle the error
    // if the file doesnt exist, 
    let result = File::open("hello.txt").unwrap_or_else(|error| {

        // if the file doesn't exist, create it
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })

        // else there is not more we can do, panic 
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
    
    
  


    // print the file, else create it and print it
    println!("{:?}", result);



    // calling the propagation method
    // read_file().unwrap();
    
   Ok(())
}



