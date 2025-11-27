use std::io;

fn main() {
    println!("Please enter the package weight(kg):");
    let mut package_input = String::new();

    io::stdin()
        .read_line(&mut package_input)
        .expect("Failed to read line");

    let package_weight: f64 = package_input
        .trim()
        .parse()
        .expect("Invalid number for package weight");

    if package_weight > 100.0 {
        println!("Package too heavy! Limit is 100kg");
    } else {
        let price = package_weight * 5.0;
        println!("Accepted! Shipping fee is {} Euros", price);
    }
}
