use std::io::{self, Read};

fn main() {
    println!("Enter your weight (kg): ");

    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input);

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight);

    println!("Weight on Mars: {:.2}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

