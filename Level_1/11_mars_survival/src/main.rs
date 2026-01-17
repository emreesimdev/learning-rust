use rand::Rng;
use std::io;

fn main() {
    let mut energy = 100;
    let mut oxygen = 100;
    let mut water = 100;
    let mut days_survived = 0;

    loop {
        days_survived += 1;
        println!(
            "Day {} on Mars. Status report: Energy: {} | Oxygen: {} | Water: {}",
            days_survived, energy, oxygen, water
        );

        if days_survived == 10 {
            println!("Rescue ship arrived! You survived Mars!🚀");
            break;
        }

        println!("Type '1' for 'Repair Solar Panels.'");
        println!("Type '2' for 'Search for Water.'");
        println!("Type '3' for 'Tend to Greenhouse.'");

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
                // Repairing Solar Panels
                energy += 20;
                oxygen -= 10;
                water -= 15;
                println!("You repaired the panels. Energy increased, but you are thirsty.");
            }
            2 => {
                // Searching Water
                energy -= 15;
                oxygen -= 10;
                let water_random = rand::thread_rng().gen_range(1..=10);
                if water_random > 7 {
                    water += 30;
                    println!("You found ice under the rocks! Water stock increased");
                } else {
                    println!("You found nothing... Just dust and cold.");
                }
            }
            3 => {
                // Tend to Greenhouse
                energy -= 10;
                oxygen += 25;
                water -= 15;
                if oxygen > 100 {
                    oxygen = 100;
                }
                println!("You tended to the plants. Oxygen levels increased, but used water.");
            }
            _ => {
                // Choice control check
                println!("Invalid choice! Please type 1, 2 or 3.");
                days_survived -= 1; // BUG FIX: Roll back the day
                continue;
            }
        }

        // Random Events

        let event = rand::thread_rng().gen_range(1..=10);

        if event == 1 {
            // Storm
            energy -= 20;
            println!("WARNING! Sandstorm hit the base. Energy lost.")
        }
        if event == 2 {
            // Radiation
            oxygen -= 20;
            println!("WARNING! Radiation leak detected. Oxygen levels dropping.");
        }

        // Death Control

        if energy <= 0 || oxygen <= 0 || water <= 0 {
            println!("Critical failure! Life support offline. GAME OVER.");
            break;
        }
    }
}
