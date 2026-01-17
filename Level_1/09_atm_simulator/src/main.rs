use std::io;

fn main() {
    let mut balance = 1000.0;

    loop {
        println!("Type '1' to see your balance.");
        println!("Type '2' to deposit money.");
        println!("Type '3' to withdraw money.");
        println!("Type '4' to exit.");

        println!("What action do you want to take?");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice_int: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice_int != 1 && choice_int != 2 && choice_int != 3 && choice_int != 4 {
            println!("Invalid choice! Please type 1, 2, 3 or 4.");
            continue;
        }

        if choice_int == 1 {
            println!("Your balance is {} Euros.", balance);
        }

        if choice_int == 2 {
            println!("What is the amount you want to deposit?");
            let mut deposit = String::new();

            io::stdin()
                .read_line(&mut deposit)
                .expect("Failed to read line");

            let deposit_float: f64 = match deposit.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            balance += deposit_float;
            println!("Your new balance is {} Euros.", balance);
        }

        if choice_int == 3 {
            println!("What is the amount you want to withdraw?");
            let mut withdraw = String::new();

            io::stdin()
                .read_line(&mut withdraw)
                .expect("failed to read line");

            let withdraw_float: f64 = match withdraw.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if withdraw_float > balance {
                println!("Insufficient balance");
            } else {
                balance -= withdraw_float;
                println!("Your new balance is {} Euros.", balance);
            }
        }

        if choice_int == 4 {
            println!("Exiting.");
            break;
        }
    }
}
