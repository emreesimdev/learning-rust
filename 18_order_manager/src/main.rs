#![allow(dead_code)]

// 1. DATA MODELLING 
// Using Debug trait to print enum values later
#[derive(Debug)]
enum OrderStatus {
    New,
    Processing,
    Shipped,
    Delivered,
}

struct Order {
    id: i32,
    customer: String,
    item: String,
    status: OrderStatus,
}

use std::io;

fn main() {
    // Dynamic list to store orders
    let mut orders: Vec<Order> = Vec::new();

    // 2. PRE-FILLING DATA
    let order1 = Order {
        id: 1,
        customer: String::from("Emre"),
        item: String::from("Coffee"),
        status: OrderStatus::New,
    };
    orders.push(order1);

    let order2 = Order {
        id: 2,
        customer: String::from("Jake"),
        item: String::from("Hot Chocolate"),
        status: OrderStatus::Processing,
    };
    orders.push(order2);

    let order3 = Order {
        id: 3,
        customer: String::from("Alice"),
        item: String::from("Cookie"),
        status: OrderStatus::Shipped,
    };
    orders.push(order3);

    let order4 = Order {
        id: 4,
        customer: String::from("Jessica"),
        item: String::from("Pizza"),
        status: OrderStatus::Delivered,
    };
    orders.push(order4);


    // 3. NEW ORDER ENTRY
    loop {
        println!("--- NEW ORDER ENTRY ---");

        let id_input = get_input("Enter Order ID (or 'q' to stop adding):");

        if id_input == "q" {
            break;
        }

        let new_id: i32 = match id_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let new_customer = get_input("Enter Customer Name:");
        let new_item = get_input("Enter Item Name:");

        let new_order = Order {
            id: new_id,
            customer: new_customer,
            item: new_item,
            status: OrderStatus::New,
        };

        orders.push(new_order);
        println!("Order added successfully! \n");
    }


    // 4. LISTING CURRENT ORDERS
    println!("---CURRENT ORDERS---");
    for order in &orders {
        println!("ID: {}", order.id);
        println!("Customer: {}", order.customer);
        println!("Item: {}", order.item);
        
        match order.status {
            OrderStatus::New => println!("Status: Order Received"),
            OrderStatus::Processing => println!("Status: Processing..."),
            OrderStatus::Shipped => println!("Status: Shipped"),
            OrderStatus::Delivered => println!("Status: Delivered"),
        }
        println!("-------------------------");
    }

    // 5. UPDATE LOGIC
    let mut target_id = -1;

    // Step A: Select ID
    loop {

        let input = get_input("Which ID do you want to update? (or 'q' to exit)");

        if input == "q" {
            break;
        }

        let update: i32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };    

        target_id = update;
        break;
    }

    // Step B: Update Status        
    if target_id != -1 {
        for order in &mut orders {
            if order.id == target_id {
                println!("Current Status is: {:?}", order.status);
                
                let status_input = get_input("What should the new status be? (New, Processing, Shipped, Delivered)");

                if status_input == "New" {
                    order.status = OrderStatus::New;
                } else if status_input == "Processing" {
                    order.status = OrderStatus::Processing;
                } else if status_input == "Shipped" {
                    order.status = OrderStatus::Shipped;
                } else if status_input == "Delivered" {
                    order.status = OrderStatus::Delivered;
                } else {
                    println!("Invalid status entered! No changes made.");
                }

                println!("Order updated successfully!");
            }
        } 
    }   

    // 6. FINAL REPORT
    println!("\n---FINAL LIST---");
    for order in &orders {
        println!("ID: {}. Status: {:?}", order.id, order.status);
    }
}

// HELPER FUNCTION
// Reads input, handles errors, and trims whitespace.
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}