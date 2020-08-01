use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let mut lives = 0;
    loop {
        println!("Guessing Game | Difficulty Select:");
        println!("1: Easy\n2: Medium\n3: Hard\n");
        let mut difficulty_select = String::new();
        std::io::stdin()
            .read_line(&mut difficulty_select)
            .expect("Failed to read the difficulty select");

        let difficulty_select: u32 = match difficulty_select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match difficulty_select {
            1 => {
                lives = 9;
                break;
            }
            2 => {
                lives = 5;
                break;
            }
            3 => {
                lives = 3;
                break;
            }
            _ => {
                break;
            }
        }
    }


    println!("Can you read my mind?");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    while lives > 0 {
        println!("Guess my number between 1 and 100: ");
        println!("Guesses remaining: {}", lives);
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lives -= 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                lives -= 1;
            }
            Ordering::Equal => {
                println!("Bingo bango bongo, you did it!");
                break;
            }
        }
    }
    if lives == 0 {
        println!("You ran out of lives! Try again.");
    }
}
