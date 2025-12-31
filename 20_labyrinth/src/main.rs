#[derive(PartialEq, Clone)]
enum Tile {
    Wall,
    Empty,
    Monster,
    Weapon,
    Trap,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Player {
    x: usize,
    y: usize,
    health_point: i32,
    has_weapon: bool,
}

fn main() {

    // 10x10 Labyrinth Map
    // 0: Wall, 1: Empty, 2: Monster, 3: Weapon, 4: Trap
    let mut map = vec![
        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Monster, Tile::Wall],
        vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Wall, Tile::Wall, Tile::Empty, Tile::Wall],
        vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall],
        vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Monster, Tile::Wall, Tile::Empty, Tile::Wall],
        vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Trap, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall], 
        vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Weapon, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Empty, Tile::Wall],
        vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Monster, Tile::Empty, Tile::Wall], 
        vec![Tile::Wall, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Empty, Tile::Trap, Tile::Empty, Tile::Empty],
        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall], 
    ];

    // Starting position of the player (1,1)
    let mut player = Player {
        x: 1,
        y: 1,
        health_point: 3,
        has_weapon: false,
    };
    
    // Game Loop
    loop {
        // Draw the map row by row 
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // Check if the player is in this cell
                if x == player.x && y == player.y  {
                    print!("P");
                } else {
                    // Draw the tile based on its type
                    match cell {
                        Tile::Wall => print!("#"),
                        Tile::Empty => print!("."),
                        Tile::Weapon => print!("W"),
                        Tile::Monster => print!("M"),
                        Tile::Trap => print!("T"),
                    }
                }
            }
            // Move to the next line after drawing a row
            println!();
        }

        // Status bar
        println!();
        println!("-----------------------");
        println!("HP: {} | Weapon {}", player.health_point, player.has_weapon);
        println!("Press 'a' (Left), 'd' (Right), 'w' (Up), 's' (Down)");
        println!("Press 'q' to quit.");

        let choice = get_input("What is your move?");

        if choice == "q" {
            println!("Goodbye");
            break;
        }

        // Determine direction based on input
        let direction =  match choice.as_str() {

        "a" => Direction::Left,
        "d" => Direction::Right,
        "w" => Direction::Up,
        "s" => Direction::Down,
        _ => { 
            println!("Invalid choice!");
            continue;
            },
        };

        // Calculate target cordinates based on direction
        let mut target_x = player.x;
        let mut target_y = player.y;

        match direction {
        Direction::Left => target_x -= 1,
        Direction::Right => target_x += 1,
        Direction::Up => target_y -= 1,
        Direction::Down => target_y += 1,
        }

        // Interection Logic: What is on the target tile?
        match map[target_y][target_x] {
        Tile::Empty => {
            player.x = target_x;
            player.y = target_y;
        },
        Tile::Wall => {
            println!("You hit the wall!");
        },
        Tile::Weapon => {
            player.has_weapon = true;
            map[target_y][target_x] = Tile::Empty; // Remove weapon from map
            player.x = target_x;
            player.y = target_y;
        },
        Tile::Monster => {
            if player.has_weapon {
                println!("You killed the monster!");
                map[target_y][target_x] = Tile::Empty; // Remove monster from map
                player.x = target_x;
                player.y = target_y;
            } else {
                player.health_point -= 1;
                println!("You've taken damage!");
            }
        },
        Tile::Trap => {
            println!("IT'S A TRAP! The floor collapsed. (You lost 2 HP)");
            player.health_point -= 2;
            map[target_y][target_x] = Tile::Empty; // Trap is triggered and gone
            player.x = target_x;
            player.y = target_y;
        },
    };

    // Lose Condition
    if player.health_point <= 0 {
        println!("Game Over! You died inside the dark labyrinth...");
        break;
        }

    // Win Condition (Exit is at x:9, y:8)
    if player.x == 9 && player.y == 8  {
        println!("VICTORY! You escape the labyrinth");
        break;
        }

    }

}    

// Helper Function for user input
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}
