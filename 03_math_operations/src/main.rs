fn main() {
    // Standart Integer Addition
    let a = 10;
    let b = 5; 
    let sum = a + b;
    println!("sum(a + b): {}", sum);
    
    // Standart Float Subtraction 
    let x = 10.5;
    let y = 2.5;
    let difference = x - y;
    println!("Difference(x - y): {}", difference);

    // Multiplication & Division
    let product = a * b;
    let quotient = a / b;
    println!("Multiplication: {}, Division: {}", product, quotient);

    // What happens if we divide 5 by 2?
    let remainder = 5 % 2;
    let integer_div = 5 / 2;

    println!("5 % 2 (Remainder): {}", remainder);
    println!("5 /2 (Integer Division): {}", integer_div);

    // MIXED TYPED ERROR EXAMPLE 
    // Uncomment the line below to see the error!
    // let error_math = a + x;

    // FIXING THE ERROR

    // We use 'as' keyword to convert integer to float 
    let fixed_math = (a as f64) + x;

    println!("Fixed Math Result: {}", fixed_math);

    // FIXING THE INTEGER DIVISION
    // To get 2.5, we must use floats (decimals) not integers

    let decimal_division = 5.0 / 2.0;

    println!("Correct Division (5.0 / 2.0): {}", decimal_division);
}
