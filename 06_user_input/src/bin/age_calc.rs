use std::io;

fn main() {
    println!("Enter your birth year (e.g., 1990):");

    let mut year_input = String::new();

    io::stdin()
        .read_line(&mut year_input)
        .expect("Failed to read input");

    // DATA TRANSFORMATION PIPELINE
    let year: i32 = year_input
        .trim() // 1. Remove the invisible \n character
        .parse() // 2. Try to convert string to number
        .expect("Please type a valid number!"); // 3. Crach nicely if input is text

    let current_year = 2025;
    let age = current_year - year;

    println!("You are approx. {} years old.", age);
}
