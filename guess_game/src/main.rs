use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Wprowadź swoją liczbę:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Nie udało się odczytać linii");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Twoja liczba: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mało!"),
            Ordering::Greater => println!("Za dużo!"),
            Ordering::Equal => {
                println!("Wygrałeś!");
                break;
            }
        }
    }
}

