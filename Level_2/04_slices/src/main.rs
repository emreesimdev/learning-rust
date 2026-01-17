fn main() {
    // 1. Create the main String (The Owner)
    // "EU-GERMANY-DATA" is stored in Heap.
    let data = String::from("EU-GERMANY-DATA");

    println!("Full Data: {}", data);

    // 2. Create Slices (The Borrowers)
    // We are not copying data. We just point to specific bytes.

    // Get "EU" (Index 0 and 1)
    // Syntax: &variable[start_index..end_index]
    let region = &data[0..2];

    // Get "GERMANY" (Starts at index 3, ends before index 10)
    let country = &data[3..10];

    // Print Slices
    println!("Region: {}", region);
    println!("Country: {}", country);
}