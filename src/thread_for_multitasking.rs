use std::thread;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread! Number: {}", i);
            // Simulate some work with sleep
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // Main thread continues
    for i in 1..5 {
        println!("Hello from the main thread! Number: {}", i);
        thread::sleep(std::time::Duration::from_millis(500));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}