#[allow(dead_code)]

// FIX 1: Add this line to enable printing for the Enum
#[derive(Debug)]

enum Color {
    Red,
    Blue,
    Black,
}

struct Car {
    model: String,
    speed: i32,
    color: Color,
}

fn main() {

    let my_car = Car {
        model: String::from("Ferrari"),
        speed: 300,
        color: Color::Red,
    };

    println!("Model: {}",my_car.model);
    println!("Speed: {}",my_car.speed);
    // FIX 2: Use {:?} instead of {} for custom types (Debug format)
    println!("Color: {:?}", my_car.color);
}