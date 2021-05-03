use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("guess value must be between 1 and 100, got: {}", value);
        }
        Guess { value: value }
    }
    pub fn value(&self) -> u32 {
        return self.value;
    }
}

fn main() {
    println!("Guess the number!");
    let mut rng = rand::thread_rng();
    let secret_number: u32 = rng.gen_range(1, 101);

    loop {
        // Collect input
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // Parse input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let g = Guess::new(guess);
        println!("you guessed: {}", g.value());

        // Check guess
        match g.value().cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("winner winner");
                break;
            }
        }
    }
}
