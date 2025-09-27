
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut lower_bound = 1;
    let mut upper_bound = 100;

    let secret_number = rand::thread_rng().gen_range(lower_bound..=upper_bound);

    println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                lower_bound = guess + 1;
            },
            Ordering::Greater => {
                println!("Too big");
                upper_bound = guess - 1; 
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        println!("The secret number is between {lower_bound} and {upper_bound}");
    }
}
