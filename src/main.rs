use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let target_number: u32 = rand::thread_rng().gen_range(0..100);
    let mut score = 0;

    loop {
        println!("Please input a number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number.");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&target_number) {
            Ordering::Less => {
                println!(
                    "That was too {}{}{}",
                    "low".red(),
                    '.',
                    " Try again!".blue()
                );
                score += 1;
            }
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                score += 1;
                println!("{}{}", "Score: ", score.to_string().blue());
                break;
            }
            Ordering::Greater => {
                println!(
                    "That was too {}{}{}",
                    "high".red(),
                    '.',
                    " Try again!".blue()
                );
                score += 1;
            }
        };
    }
}
