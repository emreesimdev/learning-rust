fn main() {
    // Financial goal configuration
    let current_savings = 5000.0;
    let target_amount = 10000.0;

    // Compare savings against the target for Germany relocation plan
    if current_savings >= target_amount {
        println!("Goal reached! Germany calling.");
    } else {
        // Calculate the missing amount dynamically inside the print macro
        println!("You need {} Euros more.", target_amount - current_savings);
    }
}