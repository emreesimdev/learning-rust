use std::io;

fn main() {
    println!("Say something, I will repeat it:");

    // 1. Create an empty String buffer
    let mut message = String::new();

    // 2. Listen to keyboard and fill the buffer
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    // 3. Print to screen (We don't use trim() here to see the effect to Enter key)
    println!("You said: {} - Squawk!", message);

    // Note: The "Squawk!" wil appear on a new line because of the hidden '\n' character.
}
