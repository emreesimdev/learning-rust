use std::io; // Import Unput/Output library

fn main() {
    println!("Please enter your username");

    // 1. Create a Mutable String Buffer
    // String::new() creates an empty, growable text variable
    let mut username = String::new();

    // 2. Read from Keyboard (Standart Input)
    // We pass the REFERENCE (&mut) of our variable so read_line can fill it
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line"); // Error handling

    // .trim() removes whitespace and new lines (\n)
    println!("Welcome back, {}!", username.trim());
}
