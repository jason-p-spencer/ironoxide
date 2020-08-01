use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Can you read my mind?");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop {
        println!("Guess my number between 1 and 100: ");
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
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo bango bongo, you did it!");
                break;
            }
        }
    }
}
