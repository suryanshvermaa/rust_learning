use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing the number!");
    // guessing a random number between 1-100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your Guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");
        let guess: u32 = match guess.trim().parse() {
            Err(_)=>{
                println!("please enter a valid input!");
                continue;
            },
            Ok(num)=>num
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too low!"),
        }
    }
}
