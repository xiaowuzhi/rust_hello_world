use rand::Rng;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("please input you guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess);
    }
}
