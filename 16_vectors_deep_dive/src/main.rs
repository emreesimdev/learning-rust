fn main(){
    println!("---Lesson 16: Vectors Deep Dive (Iteration, Options, and Safety)---");

    // 1. ITERATION: READING VALUES
    // We create a vector of student scores.
    let scores = vec![85, 90, 78, 92];

    println!("\n1. Iterating over elements (Immutable Borrow):");
    // We use '&scores' to borrow the vector.
    // IF we used 'scores' directly (without &), ownership would move the loop
    // and we couldn't use the vector afterwards.
    for score in &scores {
        println!("Student Score: {}", score);
    }

    // MUTABLE ITERATION: MODIFYING VALUES
    // Let's add a 'curve' to the grades (add +5 points to each).
    // The vector must be mutable
    let mut grades = vec![50, 60, 70];

    println!("\n2. Modifying elements in a loop (Mutable Borrow)");
    for grade in &mut grades {
        // We dereference (*) the values to modify the actual data in memory.
        *grade += 5;
    }
    println!("Updated Grades: {:?}", grades);

    // 3. SAFE ACCESS with .get()
    // Direct indexing like grades[10] will panic (crash) if the index doesn't exist.
    // .get() returns an Option<T> (Some or None), which is safer.
    println!("\n3. Safe Access using .get()");

    let index = 10; // An index that definitely doesn't exist
    match grades.get(index) {
        Some(grade) => println!("The grade at index {} is: {}", index, grade),
        None => println!("No grade found at index {} (Safe handling)", index),
    } 

    // 4. HANDLING .pop() with MATCH
    // .pop() removes the last element and returns an Option.
    // We must handle the case where the vector is empty.
    println!("\n4. Handling .pop() safely");

    let mut tasks = vec!["Task 1"];

    // First pop: Should return Some("Task 1")
    match tasks.pop() {
        Some(task) => println!("Completed: {}", task),
        None => println!("No tasks left!"),
    }

    // Second pop: Vector is now empty, should return None
    match tasks.pop() {
        Some(task) => println!("Completed: {}", task),
        None => println!("Task list is already empty!"),
    }
}