use std::io;
use rand::Rng;


fn main() {
    println!("Guess the Number");
    let random_number = gen_random();
    println!("The secret number is: {}", random_number);

    let guess = get_guess();
    println!("guess: {}", guess);
}

fn get_guess() -> String {
    
    println!("Please Input Your Guess");
    println!("--------");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess).expect("Failed to read line");
    return guess;
}

fn gen_random() -> u64 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    return secret_number;
}