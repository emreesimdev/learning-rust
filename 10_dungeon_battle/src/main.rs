use rand::Rng;
use std::io;

fn main() {
    let mut player_health = 100;
    let mut monster_health = 100;

    loop {
        println!(
            "Status report! Your health is {}. Monster's health is {}",
            player_health, monster_health
        );

        println!("Type '1' for attack.");
        println!("Type '2' for heal.");
        println!("Type '3' for run.");

        println!("What is your choice?");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice_int: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice_int != 1 && choice_int != 2 && choice_int != 3 {
            println!("Invalid choice! Please type 1, 2 or 3.");
            continue;
        }

        if choice_int == 1 {
            let damage = rand::thread_rng().gen_range(5..=12);
            monster_health -= damage;
            println!("You dealt {} damage to the Monster.", damage);
            if monster_health <= 0 {
                println!("You have slain the Monster.");
                break;
            }
        }

        if choice_int == 2 {
            let heal = rand::thread_rng().gen_range(10..=15);
            player_health += heal;
            if player_health > 100 {
                player_health = 100;
            }
            println!(
                "You healed for {}.Your health is {} now.",
                heal, player_health
            );
        }

        if choice_int == 3 {
            let run = rand::thread_rng().gen_range(1..=10);
            if run > 8 {
                println!("You escaped succesfully!");
                break;
            } else {
                println!("Escape Failed! The Monster caught you");
            }
        }

        let monster_damage = rand::thread_rng().gen_range(10..=20);
        player_health -= monster_damage;
        println!("The Monster dealt {} damage to you!", monster_damage);
        if player_health <= 0 {
            println!("You are dead... GAME OVER.");
            break;
        }
    }
}
