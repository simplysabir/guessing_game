use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You Guessed : {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You Guessed it correctly");
                break;
            }
            Ordering::Greater => println!("Too Large"),
        }
    }
}
