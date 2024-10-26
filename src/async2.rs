use std::time::Duration;
use tokio::time::sleep; // A timer provided by the `tokio` async runtime

// This is an async function that simulates a long task
async fn read_book() -> String{
    println!("Start reading...");
    sleep(Duration::from_secs(2)).await; // Simulate waiting
    println!("Finished reading the book!");
    return String::from("Read")
}

async fn do_home_work(next: String) -> String{
    println!("Start homework...");
    sleep(Duration::from_secs(4)).await; // Simulate waiting
    println!("Finished homework!");
    let mut init_text = String::from(next);
    init_text.push_str(" and Did My Homework!");
    return init_text
}

// Another async function
async fn cook_lunch(first: String) -> String{
    println!("Start cooking...");
    sleep(Duration::from_secs(3)).await; // Simulate waiting
    println!("Lunch is ready!");
    let mut init_text = String::from(first);
    init_text.push_str(" and Cooked");
    return init_text
}

#[tokio::main]
async fn main() {
    //  // Run two tasks at the same time
    // let task1 = task1();
    // let task2 = task2();

    // // Wait for both tasks to complete
    // tokio::join!(task1, task2);

    let task1 = read_book().await; // Start the book task
    let task2 = cook_lunch(task1).await; // Start the cooking task
    let task3 = do_home_work(task2).await; // Start homework
    println!("Doing both tasks asynchronously...");
    
    // // Await both tasks to complete
    // task1.await;
    // task2.await;
    // task3.await;
    
    println!("All tasks are done!");
    println!("I {}", task3)
}


