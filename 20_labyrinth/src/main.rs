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

        if choice == "q" {
            println!("Goodbye");
            break;
        }

    let direction =  match choice.as_str() {

        "a" => Direction::Left,
        "d" => Direction::Right,
        _ => { 
            println!("Invalid choice!");
            continue;
        },
    };

    let target = match direction {
        Direction::Left => {
            if player.position > 0 {
                player.position -1
            } else {
                continue;
            }
        },
        Direction::Right => {
            if player.position < map.len() - 1 {
                player.position + 1
            } else {
                continue;
            }
        },
    };

    match map[target] {
        Tile::Empty => {
            player.position = target;
        },
        Tile::Wall => {
            println!("You hit the wall!");
        },
        Tile::Weapon => {
            player.has_weapon = true;
            map[target] = Tile::Empty;
            player.position = target;
        },
        Tile::Monster => {
            if player.has_weapon {
                println!("You slain The Monster!");
                map[target] = Tile::Empty;
                player.position = target;
            } else {
                player.health_point -= 1;
                println!("You've taken damage!");
            }
        },
    };

    if player.health_point <= 0 {
        println!("Game Over! You died.");
        break;
    }

    if player.position == map.len() -1 {
        println!("You managed to escape the labyrinth!");
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
