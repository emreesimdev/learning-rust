// This line tells Rust to ignore unused variables/codes.
#![allow(dead_code)]
// STEP 1: DESIGN (Outside main)
// Creating our own data type.
// This is a 'Template'. No real player exists yet.

struct Player {
    name: String,   // Player's name
    health: i32,    // Health points
    damage: i32,    // Attack power
    has_key: bool,  // Does the player have the key?
}

// Defining directions. Restricted to these options only.
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    // STEP 2: CREATION (Instance)
    // Creating a real player using the template above.
    
    let mut my_player = Player {
        name: String::from("Ghost"), // Using 'from' since it's a String
        health: 100,
        damage: 15,
        has_key: false,
    };

    // STEP 3: USAGE
    // Accessing data using the '.' (dot) operator.
    println!("Player Name: {}", my_player.name);
    println!("Health Status: {}", my_player.health);

    // Reduce health (Simulating taking damage)
    my_player.health -= 20;
    println!("Took damage! New Health: {}", my_player.health);

    // Picking a direction
    let move_dir = Direction::North;

    // Checking the direction (Match works great with Enums)
    match move_dir {
        Direction::North => println!("Heading North..."),
        Direction::South => println!("Heading South..."),
        _ => println!("Other directions..."),
    }
}