use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Try to guess the number. Let's see how smart you are!");
    println!("Only numbers from 1 to 100.");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number if you want to keep playing. Duh.");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
        println!("Only numbers from 1 to 100 are allowed.");
        continue;
    }


        println!("Your guess was: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You got it! Congratulations for... nothing.");
                break;
            }
        }
    }
}
