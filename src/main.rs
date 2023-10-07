use std::env::{args, Args};

fn calculator(x: f32, y: f32, op: &str) -> f32 {
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => panic!("Unknown operator"),
    }
}

fn main() {
    let mut args: Args = args();

    let first_arg: String = args.nth(1).expect("No arguments given");
    let operator: String = args.nth(0).expect("No operator given");
    let second_arg: String = args.next().expect("No second argument given");

    let first_arg: f32 = first_arg
        .parse::<f32>()
        .expect("First argument is not a number");
    let second_arg: f32 = second_arg
        .parse::<f32>()
        .expect("Second argument is not a number");

    let result: f32 = calculator(first_arg, second_arg, &operator);

    println!("{} {} {} = {}", first_arg, operator, second_arg, result);
}
