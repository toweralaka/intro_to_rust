fn main() {
    let mut text = String::from("Hi");
    
    // Call your function here to modify the text
    // and print the new length
    let the_len = format!("{}", modify_and_return_length(&mut text));
    println!("The modified version is {} with length {}", text, the_len);
}

fn modify_and_return_length(text: &mut String) -> usize {
    // Modify the string and return its length
    text.push_str(", Ola");
    return text.len()
}
