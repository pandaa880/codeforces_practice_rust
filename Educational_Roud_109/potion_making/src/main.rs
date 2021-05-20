use std::io;
use std::process;

fn main() {
    let essence_values = read_input();
    let result = min_steps(essence_values);

    for r in result {
        println!("{}", r);
    }
}

// TODO: fix this solution (wrong answer)
fn min_steps(values: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();

    for i in values {
        let val = i as u32;
        if 100 % val == 0 {
            result.push(100 / val);
        } else {
            result.push(100);
        }
    }

    result
}

fn read_input() -> Vec<u32> {
    let n = read_number();
    let mut essence_values = Vec::new();
    for _i in 0..n {
        let v = read_number();
        essence_values.push(v);
    }

    essence_values
}

fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let digit: u32;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        }
        Err(_err) => {
            println!("Please enter valid number.");
            process::exit(1);
        }
    }

    digit
}
