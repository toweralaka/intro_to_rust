// we add lifetime `'a` to longest_name because the arguments are borrowed 
// we need toy1 and toy2 to remain available for as long as longest_name is called
fn longest_name<'a>(toy1: &'a str, toy2: &'a str) -> &'a str {
    if toy1.len() > toy2.len() {
        toy1
    } else {
        toy2
    }
}

fn main() {
    let toy1 = String::from("robot");
    let toy2 = String::from("superhero");

    // longest "borrows" toy1 and toy2 in calling the longest_name function
    let longest = longest_name(&toy1, &toy2);
    println!("The longest toy name is: {}", longest);
}


// Adding the lifetime `'a` to `longest_name` specifies that the 
// returned reference will be valid as long as both `toy1` and
// `toy2` are valid (i.e., they share the same lifetime).
// This is necessary because `toy1` and `toy2` are borrowed references, 
// and Rust needs to ensure they won’t be dropped while `longest_name`
// is still in use.
fn longest_name<'a>(toy1: &'a str, toy2: &'a str) -> &'a str {
    if toy1.len() > toy2.len() {
        toy1
    } else {
        toy2
    }
}

fn main() {
    let toy1 = String::from("robot");
    let toy2 = String::from("superhero");

    // Here, `longest` borrows references to `toy1` and `toy2`
    // through the `longest_name` function.
    // Thanks to the lifetime annotation `'a`, Rust guarantees
    // that `longest` cannot outlive `toy1` or `toy2`.
    let longest = longest_name(&toy1, &toy2);
    println!("The longest toy name is: {}", longest);
}
