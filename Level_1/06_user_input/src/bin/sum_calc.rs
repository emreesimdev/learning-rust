use std::io;

fn main() {
    println!("Enter the first number:");
    let mut input1 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    // SECOND NUMBER
    println!("Enter the second number:");
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // CONVERSION AND CALCULATION
    // We use f64 to allow decimal numbers
    let num1: f64 = input1.trim().parse().expect("Invalid number for input 1");
    let num2: f64 = input2.trim().parse().expect("Invalid number for input 2");

    let total = num1 + num2;

    println!("Total result: {}", total);
}
