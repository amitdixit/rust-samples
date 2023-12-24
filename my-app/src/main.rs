use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // number_guessing_game();

    //   shadow_example();
    let s1 = String::from("hello");
    let (s2, len) = calculate_str_length(s1);
    println!("The length of {} is {}.", s2, len);

    let s11 = String::from("hellk,jhkjho");
    let len11 = calculate_str_length1(&s11);
    println!("The length of {} is {}.", s11, len11);

    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("{}", s3);

    let mut name_str = String::from("Amit Dixit");

    let word = get_first_word(&name_str);

    println!("{}", word);
}

fn calculate_str_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_str_length1(s: &String) -> (usize) {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn shadow_example() {
    let x = 5;
    let y = x;

    println!("{} {}", x, y);

    let name = "amit";

    let new_name = name;

    println!("{} {}", name, new_name);

    let name2 = String::from("value");

    let new_name2 = name2.clone();

    println!("{} {}", name2, new_name2);
}

fn number_guessing_game() {
    println!("{}", "Guess the number".yellow());
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("secret_number: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        println!("You guessed: {}", guess_number);

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            } // continue to the next iteration of the loop.
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".blue()),
            Ordering::Equal => {
                println!(
                    "{}{}",
                    "You win! secret_number =".green(),
                    secret_number.to_string().green()
                );
                break;
            }
            _ => println!("Something went wrong"),
        };
    }
}
