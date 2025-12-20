struct Player {
    position: usize,
}

fn main() {

    let map = vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.',];

    let mut player = Player {
        position: 4,
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
        println!("Press 'd' to move right.");
        println!("Press 'q' for quit.");

        let choice = get_input("What is your move?");

        match choice.as_str() {
            "a" => {
                if  player.position > 0 {
                   let target = player.position - 1;

                   if map[target] != '#' {
                       player.position = target;
                   } else {
                       println!("You hit a wall");
                   }
                } 
            }

            "d" => {
                let target = player.position + 1;

                if target < map.len() {
                    
                    if map[target] != '#'  {
                        player.position = target;
                    } else {
                        println!("You hit a wall!");
                    }
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

        if player.position == map.len() - 1 {
            println!("You managed to escape the labyrinth");
            break;
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}
