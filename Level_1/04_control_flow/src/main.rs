fn main() {
    // Age of the applicant
    let age = 16;

    // Check if age is enough for driving license in Germany 
    if age >= 18{
        println!("You can get a driver's license in Germany!");        
    } else {
        println!("You are too young. Please wait {} more years.", 18 - age);
    } 

    // Multiple conditions with 'else if'
    let german_level = "A2";

    if german_level == "B2" {
        println!("Language level is perfect for job search.");
    } else if german_level == "C1" {
        println!("You are practically a native speaker!");
    } else {
        println!("Current level: {}. You need to study more for B2.", german_level);
    }
}
