use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::channel;

fn main() {
    


    // creating a channel
    let (tx, rx) = channel();
    let tx_clone = tx.clone();


    // creating a thread that sends data to the channel
    let h1 = spawn(move || {
        let vals = vec![
            String::from("this"),
            String::from("is "),
            String::from("the"),
            String::from("second thread"),
        ];


        for data in vals {
            tx_clone.send(data).unwrap();
            sleep(Duration::from_millis(1));
        }
    });

    let h2 = spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];


        for data in vals {
            tx.send(data).unwrap();
            sleep(Duration::from_millis(1));
        }
    });


    // receiving data from the channel
    for rec in rx {
        println!("Got: {}", rec);
    }


    // code that runs in the main thread
    println!("End of main thread");

    // running the thread until the end of the main thread
    h1.join().unwrap();
    h2.join().unwrap();
}
