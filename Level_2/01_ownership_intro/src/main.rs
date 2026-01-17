fn main() {
    // --- PART 3: THE SOLUTION (Borrowing) ---
    // This is the most efficient way.
    // I am not moving ownership, I am just lending it via reference (&).

    let s1 = String::from("EmreEsim");

    // I pass the reference (&s1), not the value itself
    calculate_lenght(&s1);

    // s1 is still valid here because I only lent it!
    println!("Main Function: '{}' is safe and sound", s1);
}

// This function takes a reference to a String.
// It can read, but it cannot take ownership.
fn calculate_lenght(s: &String) {
    println!("Helper Function: '{}' has {} characters.", s, s.len());
}

/* ----------------------------------------------
--- MY LEARNING NOTES (HISTORY) ---
-------------------------------------------------

--- PART 1: THE CRASH (Move Semantics) ---
// Integers are simple types (Stack). Copy is cheap and automatic.
let x = 5;
let y = x; // Rust creates a copy. Both are valid.
println!("Stack: x = {}. y = {}", x, y);

// Strings are complex types (Heap).
let s1 = String::from("Rust");

// Ownership is moved from s1 to s2.
// Rust does not copy the heap data (too expensive).
let s2 = s1;

// CRASH EXPECTED HERE:
// s1 is dead. Accessing it causes an error.
// println!("Heap: s1 = {}", s1);

-----------------------------------------------------------------------

--- PART 2: THE CLONE (Deep Copy) ---
let s1 = String::from("Rust");

// This creates a fully copy of the heap data.
// It works, but is uses double memory (Expensive for big data).
let s2 = s1.clone();

println!("Clone: s1 {}, s2 = {}", s1, s2);

------------------------------------------------------------------------
*/