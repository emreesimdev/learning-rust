struct Player {
    position: usize,
    name: String,
}

use std::io; 

fn main() {

    let map = vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.',];

    let mut player = Player {
        position: 4,
        name: "Emre".to_string(),
    };
    
    loop {

        for (i, cell) in map.iter().enumerate() {
        if i == player.position {
            print!("P");
        } else {
            print!("{}", cell);
        }
    }

        println!();
        println!("-----------------------");

        println!("Press 'a' to move left.");
        println!("Press 'd to move right.");
        println!("Press 'q' for quit.");

        let choice = get_input("What is your move?");

        match choice.as_str() {
            "a" => {
                if  player.position > 0 {
                    player.position -= 1;
                }
            }

            "d" => {
                if player.position < map.len() - 1 {
                    player.position += 1;
                }
                if player.position == map.len() -1 {
                    println!("You managed to escape the labyrinth.");
                    break;
                }
            }

            "q" => {
                println!("Goodbye");
                break;
            }

            _ => {
                println!("Invalid choice!");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}
