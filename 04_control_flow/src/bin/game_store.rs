fn main() {
    // Gaming budget configuration
    let wallet = 63.5;
    let game_price = 77.99;

    // Check availability
    if wallet >= game_price {
        println!("Game Purchased successfully!");
    } else {
        // {:.2} formats the number to 2 decimal places (14.49 instead of 14.4900001)
        println!("Insufficient balance. You need {:.2} more.", game_price - wallet);
    }
}