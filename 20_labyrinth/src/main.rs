struct Player {
    position: usize,
    name: String,
}

fn main() {

    let mut map = vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.',];

    let mut player = Player {
        position: 4,
        name: "Emre".to_string(),
    };
    
    for (i, cell) in map.iter().enumerate() {
        if i == player.position {
            print!("P");
        } else {
            print!("{}", cell);
        }
    }

    println!();

    loop {
        println!("Press 'a' to move left.");
        println!("Press 'd to move right.");
        println!("Press 'q' for quit.");

        let choice = get_input("What is your move?");

        match choice.as_str() {
            "a" => {
                player.position -= 1;
            }

            "d" => {
                player.position += 1;
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
