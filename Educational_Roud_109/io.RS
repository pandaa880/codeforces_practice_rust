use std::io;
use std::process;

fn main() {
    let (n, ppc, avc) = read_input();
    let result = max_advertisement(n, ppc, avc);

    println!("{}", result);
}

fn max_advertisement(n: u32, profit_per_clicj: Vec<i64>, average_clicks: Vec<i64>) -> i64 {
    // go through 1 to n

    // select a[i] which is max from profit_per_click

    // select b[i] which is max from average_clicks

    // a[i] * b[i]

    // add to sum
    33
}

fn read_input() -> (u32, Vec<i64>, Vec<i64>) {
    let n = read_number();
    let ppc = read_numbers(n);
    let avc = read_numbers(n);

    (n, ppc, avc)
}

fn read_numbers(n: u32) -> Vec<i64> {
    let mut input = String::new();
    let mut nums: Vec<i64> = Vec::new();
    io::stdin()
        .read_line(&mut input)
        .expect("User input failed");
    let words = input.split_whitespace();

    for word in words.enumerate() {
        let num: i64 = (word.1).parse().unwrap();
        nums.push(num);
    }

    println!("{:?}", nums);

    if nums.len() < n as usize || nums.len() > n as usize {
        panic!("Please enter {} numbers separated by space", n);
    }

    nums
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
