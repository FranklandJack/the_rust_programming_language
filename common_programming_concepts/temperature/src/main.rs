use std::io;

fn main() {
    println!("Enter a temperature in fahernheit:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input: f64 = input.trim().parse().expect("Please type a real number!");
    println!("{}F is {}C", input, fahrenheit_to_celsius(input));
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    return (x - 32.0) * 5.0 / 9.0;
}
