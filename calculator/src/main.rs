use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    // size param should be 0 as args is mutable
    let operator = args.nth(0).unwrap().chars().next().unwrap(); // get available operater
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);
    println!("{:?}", output(first_number, operator, second_number, result));
}

// operating
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used.")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
