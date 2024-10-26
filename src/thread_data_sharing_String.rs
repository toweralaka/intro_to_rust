use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a counter wrapped in Arc and Mutex
    let start_name = Arc::new(Mutex::new(String::from("Ade")));
    println!("The name starts at: {}", *start_name.lock().unwrap());

    // Create three helper threads
    let name1 = Arc::clone(&start_name);
    let thread1 = thread::spawn(move || {
        let mut name = name1.lock().unwrap();
        name.push_str("ola");
        println!("Thread 1 changed name to {}", name);
    });

    let name2 = Arc::clone(&start_name);
    let thread2 = thread::spawn(move || {
        let mut name = name2.lock().unwrap();
        // Complex types (like String) can have methods called
        // directly because Rust automatically handles the
        // dereferencing when using methods (like push_str).
        name.push_str("olu");
        println!("Thread 2 changed the name to {}", name);
    });

    let name3 = Arc::clone(&start_name);
    let thread3 = thread::spawn(move || {
        let mut name = name3.lock().unwrap();
        name.push_str("ona");
        println!("Thread 3 changed the name to {}", name);
    });
    // Wait for all threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();

    // Print the final value of the name
    println!("Final name is: {}", *start_name.lock().unwrap());
}
