use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Enter a number");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //&mut guess means guess is passed as a raference

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Uh-oh, this isn't a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ooof a bit too low"),
            Ordering::Greater => println!("Ah, a bit too high"),
            Ordering::Equal => {
                println!("Perfecto!");
                break;
            }
        }
    }
}
