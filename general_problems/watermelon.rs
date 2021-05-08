use std::io;
use std::process;

fn main() {
  let weight: u16 = read_number();

  if weight > 2 && weight % 2 == 0 {
    println!("YES");
  } else {
    println!("NO");
  }

}

fn read_number() -> u16 {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let digit: u16;

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