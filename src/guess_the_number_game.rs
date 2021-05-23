use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn run() {
    println!("Guess the Number!");
    println!("Input your number here: ");

    let secret_number: u32 = rand::thread_rng()
        .gen_range(1..101);

    loop {
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Can't Readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a Number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}