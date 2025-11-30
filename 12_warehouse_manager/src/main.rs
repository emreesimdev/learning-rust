use rand::Rng;
use std::io;

fn main() {
    const CAPACITY: i32 = 1000;
    let mut stock = 0;
    let mut cash = 0.0;

    loop {
        // The Workflow
        let incoming_load = rand::thread_rng().gen_range(1..=300);

        println!("Status: Stock: {} / {} | Cash: {} €", stock, CAPACITY, cash);
        println!("A Truck arrived with {} units of cargo.", incoming_load);

        // Input & Logic
        println!("Type '1' for 'Accept Cargo'");
        println!("Type '2' for 'Reject Cargo'");
        println!("Type '3' for 'Dispatch/Sell'");

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

        // Actions
        match choice_int {
            1 => {
                // Accept Cargo
                if stock + incoming_load <= CAPACITY {
                    stock += incoming_load;
                    println!("Cargo Accepted.");
                } else {
                    println!("Warehouse is full! Cannot accept.");
                }
            }
            2 => {
                //Recejt Cargo
                println!("Truck sent away.");
            }
            3 => {
                // Dispatch/Sell
                if stock == 0 {
                    println!("Warehouse is empty, nothing to sell!");
                } else {
                    let profit = (stock as f64) * 2.5;
                    cash += profit;
                    stock = 0;
                    println!(
                        "Dispatched all goods. Profit: {:.2} € | Total Cash: {:.2} €",
                        profit, cash
                    );
                }
            }
            _ => {
                // Choice Control Check
                println!("Invalid choice! Please type 1, 2 or 3.");
                continue;
            }
        }

        // Winning Condition
        if cash >= 5000.0 {
            println!("Target Reached! You are a Logistic Master! 🚛💰");
            break;
        }
    }
}
