// indicate mutability of argument
fn grow_name(mut a_name: String)->String{
    a_name.push_str("ola"); // update is in place so the return value is ().
    // Therefore, a_name needs to be returned
    return a_name
}

fn greet_user(the_name: &str){
    println!("Hello, world, {}!", the_name);
}

fn main() {
    let name = "Adefe";
    let mut lengthen_name = String::from("Ade");
    let fixed_name = "Ola";
    greet_user(name);
    greet_user(fixed_name);
    greet_user(&lengthen_name);
    let mut counter = 1;
    // for item in 1..6 {
    //     lengthen_name = grow_name(lengthen_name);
    //     // borrow variable and use in function call
    //     // since greet_user expects &str not String
    //     greet_user(&lengthen_name);
    // }.
    // // println!("checking...{}", counter < 6);
    // while counter < 6{
    //     counter += 1; // update counter
    //     lengthen_name = grow_name(lengthen_name);
    //     // borrow variable and use in function call
    //     // since greet_user expects &str not String
    //     greet_user(&lengthen_name);
    // }
    loop {
        counter += 1; // update counter
        lengthen_name = grow_name(lengthen_name);
        // borrow variable and use in function call
        // since it will be reused down the loop
        // greet_user expects &str not String
        greet_user(&lengthen_name);
        if counter == 6{
            break;
        }
    }
}
