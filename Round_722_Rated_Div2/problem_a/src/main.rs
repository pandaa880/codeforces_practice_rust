/* Eshag loves Big Arrays */

use std::io;
use std::process;

fn main() {
    let mut t = read_number();
    let mut results = Vec::new();

    while t != 0 {
        let n = read_number();
        let nums = read_numbers(n);
        let mut sorted_numbers = sort_numbers(nums);
        let r = max_delete(&mut sorted_numbers);

        results.push(r);
        t -= 1;
    }
    for i in 0..results.len() {
        println!("{}", results[i]);
    }
}

// 2 pointer approach
fn max_delete(arr: &mut Vec<u64>) -> u64 {
    let mut count: u64 = 0;
    let mut pt1: usize = 0;
    let mut pt2: usize = arr.len() - 1;

    while pt1 != pt2 {
        let avg: i64 = ((arr[pt1] + arr[pt2]) as i64) / 2;

        if arr[pt2] > avg as u64 && arr[pt2] != 0 {
            arr[pt2] = 0;
            pt2 -= 1;
            count += 1;
            continue;
        }

        if arr[pt1] > avg as u64 && arr[pt1] != 0 {
            arr[pt1] = 0;
            pt1 += 1;
            count += 1;
            continue;
        }

        if arr[pt1] == arr[pt2] {
            if pt2 - pt1 == 1 {
                pt2 = pt1
            } else {
                pt2 -= 1;
                pt1 += 1;
            }
        }
    }

    count
}

// can improve sort algo
// used bubble sort because constraints are as low as 100
fn sort_numbers(arr: Vec<u64>) -> Vec<u64> {
    let mut a = arr.clone();

    for i in 0..a.len() {
        for j in 0..(a.len() - i - 1) {
            if a[j] > a[j + 1] {
                let t = a[j];
                a[j] = a[j + 1];
                a[j + 1] = t
            }
        }
    }

    a
}

fn read_numbers(l: u64) -> Vec<u64> {
    let mut input = String::new();
    let mut nums: Vec<u64> = Vec::new();

    io::stdin()
        .read_line(&mut input)
        .expect("User input failed");
    let words = input.split_whitespace();

    let mut count = 0;

    for num in words.enumerate() {
        let t: u64 = (num.1).parse().unwrap();
        nums.push(t);
        count += 1;
    }

    if count != l {
        panic!("Please enter valid array items");
    }

    nums
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
            println!("Please enter valid number.");
            process::exit(1);
        }
    }

    digit
}
