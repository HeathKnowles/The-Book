use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::{self, Colorize};

fn main() {
    print!("Guessing Game");

    let secret_number = rand::rng().random_range(1..101);
    loop {
        println!("Please Input Your Guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess :u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };
        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            },
        }
    }
}
