fn main() {
    println!("--- Exclusive Range (1..5) ---");
    // 5 is NOT included
    for number in 1..5 {
        println!("The number is {}", number);
    }

    println!("\n--- Inclusive Range (1..=5) ---");
    // 5 IS included
    for level in 1..=5 {
        println!("Level {} cleared!", level);
    }
}