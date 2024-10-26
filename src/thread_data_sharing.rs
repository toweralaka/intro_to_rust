use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a counter wrapped in Arc and Mutex
    let counter = Arc::new(Mutex::new(0));

    // Create two helper threads
    let counter1 = Arc::clone(&counter);
    let thread1 = thread::spawn(move || {
        let mut num = counter1.lock().unwrap();
        // Primitive types (like i32) require dereferencing (*)
        // when locked from a Mutex to modify the value.
        *num += 1;
        println!("Thread 1 incremented the counter to {}", num);
    });

    let counter2 = Arc::clone(&counter);
    let thread2 = thread::spawn(move || {
        let mut num = counter2.lock().unwrap();
        *num += 1;
        println!("Thread 2 incremented the counter to {}", num);
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}