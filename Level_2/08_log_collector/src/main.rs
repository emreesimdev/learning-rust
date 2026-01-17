#[derive(Debug)]
struct Logger {
    // A list to keep text messages 
    logs: Vec<String>,
}

impl Logger {
    // Constructor: Starts with an empty list
    fn new() -> Logger {
        Logger {
            logs: Vec::new(),
        }
    }

    // Method to add a new log
    // Using 'push' to add items to the Vector
    fn add_log(&mut self, message: String) {
        self.logs.push(message);
    }

    // Method to read all logs
    // Using a 'for' loop to visit every item in the list
    fn show_logs(&self) {
        for log in &self.logs  {
            println!("LOG: {}", log);
        }
    }
}

fn main() {
    let mut logger = Logger::new();

    // Adding data to our list
    logger.add_log(String::from("System Started!"));
    logger.add_log(String::from("Error 404 Not Found"));
    logger.add_log(String::from("User Logged in"));

    println!("--- LOG RECORDS ---");

    // Printing the list content
    logger.show_logs();
}   