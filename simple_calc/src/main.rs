use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("welcome to the most basic calculator ever");
    println!("-----------------------------------------");

    loop {
        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut operator = String::new();

        print!("what is the first number?: ");
        read(&mut n1);

        print!("what is the second number?: ");
        read(&mut n2);

        print!("what operation would you like to do? [+-*/]: ");
        read(&mut operator);

        let n1: f32 = n1.trim().parse().unwrap();
        let n2: f32 = n2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        // use vector in advanced calc
        let operators = String::from("+-*/");

        if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }

        let result = match operator {
            '+' => n1 + n2,
            '-' => n1 - n2,
            '*' => n1 * n2,
            '/' => n1 / n2,
            _ => panic!("error in operator")
        };

        println!("the result of {} {} {} = {}", n1, operator, n2, result);
    }
}
