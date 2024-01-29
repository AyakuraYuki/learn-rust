use std::cmp::Ordering;
use std::io;
use rand;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("I'm holding a secret number, try to guess it.");

    let mut retry = 1;

    while retry <= 10 {
        let mut input = String::new();

        println!("[{retry}] Please input your answer:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your input!");

        let guess: i32 = input.trim().parse().expect("Please type a number!");

        println!("You're answering {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too large!!!"),
            Ordering::Equal => {
                println!("Bingo!!!");
                return;
            }
            Ordering::Less => println!("Too small!!!"),
        }

        retry += 1;
        println!()
    }

    println!("Game over!! The secret number is {secret_number}")
}
