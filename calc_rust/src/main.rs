use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    // hold planet surface mass
    let mut planets: HashMap<&str, f32> = HashMap::new();
    planets.insert("Mercury", 0.38);
    planets.insert("Venus", 0.91);
    planets.insert("Earth", 1.0);
    planets.insert("Mars", 0.38);
    planets.insert("Jupiter", 2.34);
    planets.insert("Saturn", 0.93);
    planets.insert("Uranus", 0.92);
    planets.insert("Neptune", 1.12);

    println!("Enter your weight (lbs): ");

    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input);

    let mass: f32 = input.trim().parse().unwrap();

    println!("\nYOUR WEIGHT ON DIFFERENT PLANETS IN OUR SOLAR SYSTEM");
    println!("----------------------------------------------------");
    for (key, value) in &planets {
        let weight: f32 = &mass * value;
        println!("{planet:<width$}{weight:>width$.2} lbs", weight=weight, planet=key, width=10);
    }
}

