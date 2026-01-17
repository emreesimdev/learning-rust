fn main() {
    // --- STEP 1: CREATE DATA ---
    // A classic long cargo tracking ID style string.
    let mut tracking_code = String::from("TR-35-LOGISTICS-X99");
    println!("Original Code: {}", tracking_code);

    // --- STEP 2: MUTATE DATA ---
    // I am sending the mutable reference to the 'cutter' function.
    shorten_code(&mut tracking_code);

    // --- STEP 3: CHECK RESULT ---
    // The original variable is now modified!
    println!("Shorten Code: {}", tracking_code);
}

fn shorten_code(text: &mut String) {
    // truncate(5) keeps the first 5 bytes, deletes the rest
    text.truncate(5);
} 