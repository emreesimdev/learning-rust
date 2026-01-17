// Adding Debug to both Enum and Struct so we can print them.
#[derive(Debug)]
enum GameClass {
    Warrior,
    Mage,
    Rogue,
}

#[derive(Debug)]
struct Character {
    name: String,
    health: i32,
    my_class: GameClass, // Using an Enum inside a Struct.
}

impl Character {
    // Constructor: Creates a new character
    fn new(name: String, my_class: GameClass) -> Character {
        Character { 
            name: name,
            health: 100, // Default health is 100.
            my_class: my_class, 
        }
    }

    // Attack method using logic based on the class
    fn attack(&self) {
        // Using 'match' is cleaner than 'if-else' for Enums.
        // It forces us to handle every possible class.
        match self.my_class {
            GameClass::Warrior => println!("{} attacks with a sword! (10 damage)", self.name),
            GameClass::Mage => println!("{} casts a fireball! (20 damage)", self.name),
            GameClass::Rogue => println!("{} stabs with a dagger! (15 damage)", self.name),
        }
    }
}

fn main() {
    let my_char = Character::new(
        String::from("Emre"),
        GameClass::Mage // Selecting a specific class
    );

    println!("Character Info: {:?}", my_char);

    // Calling the attack method
    my_char.attack();
}