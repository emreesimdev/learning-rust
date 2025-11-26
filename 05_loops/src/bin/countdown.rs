fn main() {
    // Starting point
    let mut count = 10;

    // While 'count' is NOT zero, keep running
    while count > 0 {
        println!("T-minus: {}", count);

        // Decrease the counter
        count -= 1;
    }

    println!("LIFTOFF! 🚀");
}