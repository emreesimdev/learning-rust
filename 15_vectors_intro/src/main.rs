fn main() {
    println!("Lesson 15: Vector Basics");

    // 1. Create an empty vector and add values
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // 2. Vector with macro
    // This creates a vector containing specific values instantly.
    let names = vec!["Alice", "Bob", "Charlie", "David"];

    println!("Numbers: {:?}", numbers);
    println!("Names: {:?}", names);

    // 3. Reading elements
    // We use reference (&) because we don't want to take ownership yet.
    // Indexing starts at 0.
    let second_name = &names[1];
    println!("Second name: {}", second_name);

    // 4. Removing the last element
    // pop() returns an option because the vector might be empty.
    let last_number = numbers.pop();
    println!("Popped number: {:?}", last_number);

    println!("Final number list: {:?}", numbers); 
}
