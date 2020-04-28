use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input you guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
