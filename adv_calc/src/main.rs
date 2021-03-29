//! scientific calculator
use std::io;

fn calc(oper: &str) {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let stdin = io::stdin();

    println!("Enter the first number: ");
    stdin.read_line(&mut num1).ok();
    let num1: i32 = num1.trim().parse().unwrap();

    println!("Enter the second number: ");
    stdin.read_line(&mut num2).ok();
    let num2: i32 = num2.trim().parse().unwrap();

    match oper {
        "Addition" => println!("{} + {} = {}", num1, num2, (num1 + num2)),
        "Subtraction" => println!("{} - {} = {}", num1, num2, (num1 - num2)),
        "Multiplication" => println!("{} * {} = {}", num1, num2, (num1 * num2)),
        "Division" => println!("{} / {} = {}", num1, num2, (num1 / num2)),
        "Remainder" => println!("{} % {} = {}", num1, num2, (num1 % num2)),
        _ => println!("no"), // shouldn't get here
    };
}

fn main() {
    let funcs_to_perform = vec![
        "Addition",
        "Subtraction",
        "Multiplication",
        "Division",
        "Square",
        "Cube",
        "Square Root",
        "Remainder"];

    let mut key = String::new();
    println!("Please enter the function you'd like to perform: \n");
    let stdin = io::stdin();

    for (i, operation) in funcs_to_perform.iter().enumerate() {
        println!("{}. {}", (i + 1), operation);
    }

    stdin.read_line(&mut key).ok(); // user input
    let key: i32 = key.trim().parse().unwrap();
    let index: usize = (key as usize) - 1;

    println!("{}", index);
    // println!("\n{}", funcs_to_perform[&index]);
    // calculate::calc(&funcs_to_perform[&index]);
}
