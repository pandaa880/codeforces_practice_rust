/* And then there were K */

use std::io;
use std::process;

fn main() {
    let numbers: Vec<u64> = read_input();
    let result: Vec<u64> = calculate_k(numbers);

    for i in 0..result.len() {
        println!("{}", result[i as usize]);
    }
}

// TODO: revisit this. need more efficient solution
fn calculate_k(numbers: Vec<u64>) -> Vec<u64> {
    let mut result = Vec::new();

    for n in numbers {
        let mut temp = n;

        for i in 1..n {
            temp = temp & (n - i);

            if temp == 0 {
                result.push((n - i) as u64);
                break;
            }
        }
    }

    result
}

fn read_input() -> Vec<u64> {
    let t = read_number();
    let mut numbers: Vec<u64> = Vec::new();
    for _i in 0..t {
        let n = read_number();
        numbers.push(n);
    }

    numbers
}

fn read_number() -> u64 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let digit: u64;

    match input.trim().parse() {
        Ok(val) => {
            digit = val;
        }
        Err(_err) => {
            println!("Please enter valid length number.");
            process::exit(1);
        }
    }

    digit
}
