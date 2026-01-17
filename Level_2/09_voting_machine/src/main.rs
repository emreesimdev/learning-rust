// Importing library for keyboard input
use std::io;

#[derive(Debug)]
struct Candidate {
    name: String,
    votes: i32,
}

#[derive(Debug)]
struct VotingSystem {
    // A list to keep all candidates
    candidates: Vec<Candidate>
}

impl VotingSystem {
    // Constructor: Creates an empty voting box
    fn new() -> VotingSystem {
        VotingSystem {
            candidates: Vec::new(),
        }
    }

    // Method to register a new candidate
    // They always start with 0 votes
    fn add_candidate(&mut self, name: String) {
        let new_one = Candidate {
            name: name,
            votes: 0,
        };

        self.candidates.push(new_one);
    }

    // Main Logic: Find the candidate and add +1 vote
    fn cast_vote(&mut self, target_name: String) {
        let mut found = false; // Flag to check if we found the person

        // Loop through the list to find the matching name
        for candidate in &mut self.candidates  {
            if candidate.name == target_name {
                candidate.votes += 1;
                println!("Vote cast for: {}", target_name);
                found = true; // We found them!
            }
        }

        // If the flag is still false, it means the name is wrong
        if !found {
            println!("Candidate '{}' not found in the list!", target_name);
        }
    }
}

fn main() {
    let mut system = VotingSystem::new();

    // Adding candidates to the system
    system.add_candidate(String::from("Emre"));
    system.add_candidate(String::from("Hans"));

    println!("--- THE VOTING PROCESS HAS BEGUN ---");

    // Infinite loop for continuous voting
    loop {
        println!("\nWho are you going to vote for? (Emre / Hans) - [Type 'q' to quit]: ");

        let vote = get_input();

        // Exit Logic
        if vote == "q" {
            println!("Voting is over. Last situation: ");
            
            // Loop to print results cleanly
            for candidate in system.candidates {
                println!("- {}: {} votes", candidate.name, candidate.votes);
            }

            break;
        }

        system.cast_vote(vote);
    }
}

// Helper funtion to handle input easily
fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}