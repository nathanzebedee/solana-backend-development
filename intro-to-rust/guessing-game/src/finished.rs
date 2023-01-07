use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub mod finished {
    use super::*;
    
    pub fn finished() {
        let randNum = rand::thread_rng().gen_range(1..101);
        println!("Guess the random number! (1, 100)");
    
        loop {
            let mut guess = String::new();
    
            io::stdin().read_line(&mut guess).expect("Failed to read line");
    
            let guess: u8 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            match guess.cmp(&randNum) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
