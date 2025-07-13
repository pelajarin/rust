use std::{cmp::Ordering, io};

use rand::{Rng, thread_rng};

fn main() {
    let mut guess = String::new();
    let secret = thread_rng().gen_range(0..=100);

    loop {
        println!("Input your number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(value) => {
                guess.clear();
                value
            }
            Err(_) => {
                println!("Not a valid number, try again");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Try higher"),
            Ordering::Greater => println!("Nope, the number are less than that"),
            Ordering::Equal => {
                println!(
                    "You guessed correctly! the secret number is {secret}, congrats byebye :)"
                );
                break;
            }
        }
    }
}
