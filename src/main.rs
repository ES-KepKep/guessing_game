use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Not a number!\n");
                continue;
            }
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big\n"),
            Ordering::Equal => {
                println!("You Win\n");
                break;
            }
        }
    }
}
