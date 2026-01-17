// Allowing debug printing
#[derive(Debug)]
struct Package {
    weight: f64,
    id: u32, 
}

impl Package {
    // Constructor: Creates a new instance of Package
    fn new(id: u32, weight: f64) -> Package {
        Package {
            id: id,
            weight: weight,
        }
    }

    // Method to reduce weight safely
    fn take_weight(&mut self, amount: f64) {
        // Check if we have enough weight
        if amount > self.weight {
        println!("Error! Not enough weight.");
        } else {
            self.weight -= amount;
            println!("Success! New weight: {}", self.weight);
        }
    }
}

fn main() {
    // Creating a mutable package because weight will change
    let mut my_package = Package::new(101, 25.5);

    println!("Initial Package Info: {:?}", my_package);

    // Scenario 1: Successful operation
    my_package.take_weight(5.5);

    // Scenario 2: Error operation (Amount > Weight)
    my_package.take_weight(50.0);

}