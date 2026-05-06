use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    println!("Guess the number!");

    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".red()),
        Ordering::Equal => {
            println!("{}", "You win!".green());
            break;
        }
    }

    }

}
