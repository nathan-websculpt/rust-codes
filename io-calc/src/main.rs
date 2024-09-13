use std::io::{self, Write, stdout, stdin};

fn take_number_input() -> i32 {
    print!("\nEnter Number: ");
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    user_input.trim().parse().expect("Please type a number!")
}

fn take_operation_input() -> char {
    print!("\nEnter '+', '-', '*', '/': ");
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    user_input.trim().chars().next().expect("Please type an operation!")
}

fn main() {
    let first_input = take_number_input();
    let operation = take_operation_input();
    let second_input = take_number_input();

    match operation {
        '+' => println!("\n\n\t{} + {} = {}", first_input, second_input, first_input + second_input),
        '-' => println!("\n\n\t{} - {} = {}", first_input, second_input, first_input - second_input),
        '*' => println!("\n\n\t{} * {} = {}", first_input, second_input, first_input * second_input),
        '/' => println!("\n\n\t{} / {} = {}", first_input, second_input, first_input / second_input),
        _ => println!("\n\n\tError: Invalid operation"),
    }
}

