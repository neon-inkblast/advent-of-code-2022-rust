use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Hello, world!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Guess a number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
