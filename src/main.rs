use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! ");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //let secret_number: u32 = random() % 100 + 1; // This is the same as the above line
    println!("The secret number is: {}", secret_number);

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        // println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("This number is small!"),
            Ordering::Greater => println!("This number is greater!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
