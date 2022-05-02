use std::io;

fn main() {
    let x: u32 = loop {
        println!("Enter a number:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break input
    };

    println!("fibonacci({}) = {}", x, fibonacci(x));
}

fn fibonacci(x: u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
