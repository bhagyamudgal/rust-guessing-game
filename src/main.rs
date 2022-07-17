use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Rust Number Guessing Game - Developed by Bhagya Mudgal");

    loop {
        println!("\nEnter 'start' to start the game.");
        println!("Enter 'quit' to exit from the game.");

        println!("\nEnter Input:");

        let mut user_input1 = String::new();

        io::stdin()
            .read_line(&mut user_input1)
            .expect("Failed to read line");

        if user_input1.trim() == "start" {
            println!(
                "\nGame has started. You will get 3 chances ony. Guess a number between 1 and 100."
            )
        } else if user_input1.trim() == "quit" {
            println!("Thanks for playing the game!");
            break;
        } else {
            println!("Wrong input! Game will exit.");
            break;
        };

        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut chance = 1;

        while chance <= 3 {
            println!("\nPlease input your guess:");

            let mut user_input2 = String::new();

            io::stdin()
                .read_line(&mut user_input2)
                .expect("Failed to read line");

            let guess: u8 = match user_input2.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Only number between 1 and 100 are accepted!");
                    continue;
                }
            };

            println!("Your guess: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Greater => println!("Your guess is too big!"),
                Ordering::Less => println!("Your guess is too small!"),
                Ordering::Equal => {
                    println!("You Won!. Your guess is right.");
                    break;
                }
            }

            chance += 1;

            if chance == 4 {
                println!("\nYou lost the game! Right guess would be {secret_number}. You can start the game again");
            }
        }
    }
}
