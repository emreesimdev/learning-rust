#[derive(PartialEq, Clone)]
enum Tile {
    Wall,
    Empty,
    Monster,
    Weapon,
}

enum Direction {
    Left,
    Right,
}

struct Player {
    position: usize,
    health_point: i32,
    has_weapon: bool,
}

fn main() {

    let mut map = vec![
        Tile::Wall,
        Tile::Weapon,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Empty,
        Tile::Monster,
        Tile::Empty,
    ];

    let mut player = Player {
        position: 4,
        health_point: 3,
        has_weapon: false,
    };
    
    loop {

        for (i, cell) in map.iter().enumerate() {
        if i == player.position {
            print!("P");
        } else {
         match cell {
            Tile::Wall => print!("#"),
            Tile::Empty => print!("."),
            Tile::Weapon => print!("W"),
            Tile::Monster => print!("M"),
        };  
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

                   if map[target] != Tile::Wall {
                       player.position = target;
                   } else {
                       println!("You hit a wall");
                   }
                } 
            }

            "d" => {
                let target = player.position + 1;

                if target < map.len() {
                    
                    if map[target] != Tile::Wall  {
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
