use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut rng = rand::thread_rng(); 
    let secret_number: u64 = rng.gen_range(1, 101);
    println!("secret number: {}", secret_number);

    // Collect input
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("failed to read line");

    let guess: u64 = guess.trim().parse()
    .expect("Please type a number!");
    println!("you guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too large"),
        Ordering::Equal => println!("winner winner"),
    }
}
