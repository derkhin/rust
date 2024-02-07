use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("\n [ 1 ] Fahrenheit to Celsius\n [ 2 ] Celsius to Fahrenheit \n [ 0 ] quit \n");
        print!("[+] Select from above options: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("[-] Invalid input");

        let converted_number: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[-] Invalid input");
            return;
            }
        };

        match converted_number {
            0 => break,
            1 => fahrenheit_to_celsius_conversion(),
            2 => celsius_to_fahrenheit_conversion(),
            _ => println!("[-] Invalid input")
        }
    }
}

fn fahrenheit_to_celsius_conversion() {
    print!("Enter a Fahrenheit value: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("[-] Invalid input");

    let fahrenheit_value: f32 = match input.trim().parse() {
        Ok(num) => fahrenheit_to_celsius(num),
        Err(_) => {
            println!("Error: Invalid input");
            return;
        }
    };

    println!("Celsius: {}", fahrenheit_value);
}

fn celsius_to_fahrenheit_conversion() {
    print!("Enter a Celsius value: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");

    let celsius_value: f32 = match input.trim().parse() {
        Ok(num) => celsius_to_fahrenheit(num),
        Err(_) => {
            println!("Error: Invalid input");
            return;
        }
    };

    println!("Fahrenheit: {}", celsius_value);
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}
