use dialoguer::Input;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        
        let guess = Input::<String>::new()
            .with_prompt("Your guess")
            .interact_text()
            .unwrap();

        let guess: u32 = match guess.trim().parse() {
            Ok(i) => i,
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
