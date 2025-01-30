use std::process::exit;
use std::io::{self, stdin, Read};
use std::fs::File;





fn main() {


    let mut buf = String::new();

    match stdin().read_line(&mut buf) {
        Ok(_) => {
            println!("You entered: {}", buf);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        }
    }

    
    let mut animals = vec!["Rabbit", "Dog", "Cat"];
    let res = len_of_last_element(&mut animals);
    match res {
        Some(len) => {
            println!("Length of last element: {}", len);
        }
        None => {
            eprintln!("Error: No last element found");
            exit(4);
        }
    }

    let tmp = read_file_contents("./tmp.txt");
    match tmp {
        Ok(contents) => {
            println!("File contents: {}", contents);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(3);
        }
    }


    


    let f1 = match File::open("file1.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(2);
        }
    };

    print!("File opened successfully: {:?}", f1);


    


    exit(0);
}



fn read_file_contents(path: &str) -> Result<String, io::Error> {

    // first check with the try operator - if there is a error then propagates the error 
    let mut file = File::open(path)?;
    let mut contents = String::new();

    // second check with the try operator - if there is a error then propagates the error
    file.read_to_string(&mut contents)?; 

    // if no error then return the contents
    Ok(contents)
}

fn len_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}