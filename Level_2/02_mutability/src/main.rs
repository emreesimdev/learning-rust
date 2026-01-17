fn main() {
    // --- CASE 1: IMMUTABLE (Default Behavior) ---
    // In Rust, variables cannot change default.
    // This is good for safety. ID numbers should not change.
    let container_id = "CONT-8855-X";
    println!("Container ID: {}", container_id);

    // If I try to change it, Rust will give an error.
    // container_id = "CONT-9999-Y"; // <--- This line would crash the program.

    // --- CASE 2: MUTABLE (Changing Values) ---
    // I need to use 'mut' keyword to allow changes.
    // Real world example: Fuel level decreases over time.

    let mut fuel_level = 100; // Tank is full.
    println!("Engine started. Fuel: {}%", fuel_level);

    // The truck is moving...
    fuel_level = 90;
    println!("Reached first stop. Fuel: {}%", fuel_level);

    fuel_level = 45;
    println!("Reached destination. Fuel: {}%", fuel_level);

    // --- CASE 3: SHADOWING (Reusing Names) ---
    // I can use the same variables name again with 'let'.
    // This is useful for converting data types.

    let load_weight = "2500"; // This is text

    // I am shadowing the previous variable. Now it is a number (integer).
    let load_weight = load_weight.len();

    println!("Data length processed: {}", load_weight);
}