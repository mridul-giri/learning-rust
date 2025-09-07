use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the the number");

    let guess_number = rand::thread_rng().gen_range(1..=100);

    // println!("the secret no is {guess_number}");

    loop {
        println!("please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&guess_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
