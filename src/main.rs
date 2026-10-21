use colored::*;
use std::io::Write;

fn main() {
    let secret_number: i32 = rand::random_range(0..=10);
    let mut guess_count: i32 = 0;
    let max_guess_count: i32 = 3;
    let mut win: bool = false;

    println!("{}", "Welcome to the guessing game!".blue());

    while guess_count < max_guess_count {
        print!("enter your guess: ");
        std::io::stdout().flush().unwrap();
        let mut raw_guess: String = String::new();
        std::io::stdin()
            .read_line(&mut raw_guess)
            .expect("Failed to read input");

        let guess: i32 = match raw_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid input, please enter a valid number".red());
                continue;
            }
        };
        if guess == secret_number {
            println!("{}", "You guessed the secret number!".green());
            win = true;
            break;
        } else {
            println!("{}", "Wrong guess".yellow());
            guess_count += 1;
        }
    }

    if win {
        println!("{}", "Congratulations! You won!".green());
    } else {
        println!("{}", "Sorry, you lost!".red());
    }
}
