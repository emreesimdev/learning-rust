// Allowing debug printing for the struct
#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Constructor: Creates a new account.
    // Balance always starts at 0.0 for safety.
    fn new(owner: String) -> BankAccount {
        BankAccount {
            owner: owner,
            balance: 0.0,
        }
    }

    // Method to add money.
    // Uses '&mut self' because we are changing the balance.
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited: {}, New Balance: {}", amount, self.balance);
    }

    // Method to take money safely.
    fn withdraw(&mut self, amount: f64) {
        // LogicÇ Check if I have enough money first.
        if amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew: {}, New Balance: {}", amount, self.balance);
        } else {
            println!("Error: Insufficient balance!");
        }
    }
}

fn main() {
    // Creating the account (Must be mutable to change balance)
     let mut my_account = BankAccount::new(String::from("Emre"));

    println!("Status: {:?}", my_account);    

    // Adding some money
    my_account.deposit(100.0);
    my_account.deposit(50.5);

    // Taking some money
    my_account.withdraw(20.0);

    // Trying to take too much (to test the logic)
    my_account.withdraw(500.0);

    println!("Last balance status {:?}",my_account);
}

