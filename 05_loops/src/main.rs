fn main() {
    let mut counter = 0;

    // Infinity loop starter 
    loop{
        counter += 1; // counter = counter + 1 shorthand
        println!("Counter is: {}", counter);

        // Stop condition 
        if counter == 5 {
            println!("Stop! Loop limit reached.");
            break; // Breaks the loop
        }
    }
}
