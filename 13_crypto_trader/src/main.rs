use rand::Rng;
use std::io;

fn main() {
    // The Portfolio
    let mut cash = 1000.0;
    let mut btc = 0.0;

    // Market Loop
    loop {
        let current_price = rand::thread_rng().gen_range(30000.0..=60000.0);

        // Dashboard
        println!("Current BTC Price: {:.2}", current_price);
        println!("Portfolio: Cash: {:.2} $ | BTC: {:.6}", cash, btc);
        println!("Total Wealth: {:.2} $", cash + (btc * current_price));

        // Trade Actions
        println!("Type '1' for 'Buy Max'");
        println!("Type '2' for 'Sell Max'");
        println!("Type '3' for 'Hold'");

        println!("What is your choice?");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice_int: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Logic
        match choice_int {
            1 => {
                // BUY
                if cash <= 0.0 {
                    println!("No Cash!");
                } else {
                    let bought_btc = cash / current_price;
                    btc += bought_btc;
                    cash = 0.0;
                    println!("Bought {:.6} BTC at {:.2} $", bought_btc, current_price);
                }
            }
            2 => {
                // SELL
                if btc <= 0.0 {
                    println!("No BTC to sell!");
                } else {
                    let sold_cash = btc * current_price;
                    cash += sold_cash;
                    btc = 0.0;
                    println!("Sold BTC for {:.2} $", sold_cash);
                }
            }
            3 => {
                // HOLD
                println!("Holding...");
            }
            _ => {
                // Choice Control Check
                println!("Invalid choice! Please type 1, 2 or 3.");
                continue;
            }
        }

        // Target
        if cash + btc * current_price >= 10000.0 {
            println!("Profit target reached! You represent Web3! 🚀");
            break;
        }
    }
}
