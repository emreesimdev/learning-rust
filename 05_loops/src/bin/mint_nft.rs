fn main() {
    // Loop from 1 to 5 (inclusive)
    for token_id in 1..=5 {
        println!("Minting NFT ID: {}", token_id);

        // Check for rare item
        // No 'else' needed here if we don't do anything otherwise
        if token_id == 5 {
            println!("WOW! You found a Rare Item! 🔥");
        }
    }

    println!("All collection minted successfully!");
}
