fn main() {
    let mut battery = 100;

    while battery > 0 {
        println!("Cleaning... Battery: {}%", battery);

        battery -= 20;
    }

    println!("Battery empty. Going to charging station.");
}
