use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("I kept a number in my mind!");

    loop {
        println!("Enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let number_guess: i32 = guess.trim().parse().expect("Invalid number");

        if number_guess < secret_number {
            println!("Too small!");
        } else if number_guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
