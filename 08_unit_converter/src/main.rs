use std::io;

fn main() {
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    loop {
        println!("Which calculation do you want to make? Type '1' or '2'");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice_int: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // SAFETY CHECK: If user types 3, 4, 99... ask again!
        if choice_int != 1 && choice_int != 2 {
            println!("Invalid choice! Please type 1 or 2.");
            continue;
        }

        println!("Enter the temperature:");
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature_float: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice_int == 1 {
            println!(
                "Celsius to Fahrenheit result is {:.2}°F",
                (temperature_float * 1.8) + 32.0
            );
        } else {
            println!(
                "Fahrenheit to Celsius result is {:.2}°C",
                (temperature_float - 32.0) / 1.8
            );
        }

        // BREAK IS HERE
        break;
    }
}
