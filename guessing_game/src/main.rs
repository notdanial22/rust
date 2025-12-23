use rand::random_range;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess Number!");
    let num = random_range(1..=10);
    let mut count: u8 = 0;

    // println!("{num} , guess");

    loop {
        count += 1;
        println!("Please enter your guess:");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read what you entered.");

        let guess: u32 = guess.trim().parse().expect("Please enter a correct number");

        println!("You guessed: {guess}");
        match guess.cmp(&num) {
            Ordering::Less => println!("Low guess."),
            Ordering::Greater => println!("High guess"),
            Ordering::Equal => {
                println!("You entered correct guess");
                println!("You found correct guess: {guess} in {count} tries.");
                break;
            }
        }

    }
}
