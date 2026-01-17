fn main() {
    let mut attempt = 0;

    loop {
        attempt += 1;
        println!("Connecting to server... Attempt: {}", attempt);

        if attempt == 4 {
            println!("Connection Established! 🚀");
            break;
        }
    }
}
