fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    let names1 = vec!["Bob", "Frank", "Ferris"];
    let names2 = vec!["Bob", "Frank", "Ferris"];

    for name in names2.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names2);

    for name in names1.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    // println!("names: {:?}", names1);
    // // FIXME ^ Comment out this line


    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}