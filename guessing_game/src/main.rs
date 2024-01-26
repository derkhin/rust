use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("----------------Guess the number between 1 to 10 ---------------------");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..=10);

        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[-] Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big! {secret_number} < {guess}"),
            Ordering::Less => println!("Too small! {secret_number} > {guess}"),
            Ordering::Equal => {
                println!("You Win !! {guess}");
                break;
            }
        }
    }
}
