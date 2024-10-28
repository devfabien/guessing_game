use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number! ");
    println!("\nPlease input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //let secret_number: u32 = random() % 100 + 1; // This is the same as the above line

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number);
}
