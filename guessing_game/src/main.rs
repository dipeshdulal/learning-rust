use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess your number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number is: {secret_number}");

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!!"),
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break out of the loop
            }
        }
    }
}
