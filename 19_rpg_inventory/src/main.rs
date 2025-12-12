#![allow(dead_code)]

// 1. DATA STRUCTURES
// Using Debug to allow printing enum variants later
#[derive(Debug)]

enum ItemType {
    Weapon,
    Armor,
    Potion,
    Misc,
}

struct Item {
    name: String,
    count: i32,
    item_type: ItemType,
}

use std::io;

fn main() {
    // Inventory List (Starts empty)
    let mut inventory: Vec<Item> = Vec::new();

    // 2. MAIN GAME LOOP
    loop {
        println!("1. Add Item");
        println!("2. View Inventory");
        println!("3. Use Item");
        println!("4. Quit");

        let choice = get_input("What is your choice?");

        match choice.as_str()  {
            "1" => {
                // Add Item
                let name_input = get_input("Enter the name of the item.");

                let count_input = get_input("Enter the count of the item.");
                let new_count_input: i32 = match count_input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    } 
                }; 
                // Get type and convert String to Enum
                let item_type_input = get_input("Enter item type (Weapon, Armor, Potion)");
                
                let selected_type =  if item_type_input == "Weapon" {
                    ItemType::Weapon
                } else if item_type_input == "Armor" {
                    ItemType::Armor
                } else if item_type_input == "Potion" {
                    ItemType::Potion
                } else {
                    println!("Unkown type! Defaulting to Misc.");
                    ItemType::Misc
                };
                
                // Create and push the new item
                let new_item = Item {
                    name: name_input,
                    count: new_count_input,
                    item_type: selected_type,
                };

                inventory.push(new_item);
                println!("Item added succesfully! \n");
            }
            "2" => {
                // View Inventory
                println!("---YOUR INVENTORY---");
                for backpack in &inventory {
                    println!("Name: {}",backpack.name);
                    println!("count: {}", backpack.count);
                    println!("Item type: {:?}", backpack.item_type);
                    println!("--------------------");
            }
        }
        "3" => {
            // Use Item
            let input = get_input("Which item number will you use?");
            
            let index: usize = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            // Safety check 
            if index < inventory.len() {
                inventory[index].count -= 1;
                if inventory[index].count == 0 {
                    inventory.remove(index);
                    println!("Item consumed and removed from inventory!");
                } else {
                    println!("Items used. {} remaining.", inventory[index].count);
                }
            }  else {
                println!("Item doesn't exist");
            }
        }
        "4" => {
            // Quit
            println!("Goodbye!");
            break;
        }
        _ => {
            println!("Invalid choice!");
        }

    }
    }

}

// HELPER FUNCTION
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}