use std::{io, time::SystemTime};
use std::str::FromStr;
use io::{stdout, stdin, Read, Write};
use gmp::mpz::Mpz;

fn new_parameter(ask_string: &str) -> String {
    println!("{}", ask_string);
    let mut parameter = String::new();
    
    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to process input.");
    parameter.trim_end().to_string()
}

fn is_odd(num: &Mpz) -> bool {
    num % 2u64 == Mpz::one()
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("Collatz thing!");
    let two = 2u64;
    let three = 3u64;
    let one = 1u64;
    
    let mut num: Mpz = Mpz::from_str(&new_parameter("What number do you want to calculate? ")).unwrap();
    let print_nums = new_parameter("Should the numbers be printed? (may extend time by multiple factors.) Y/N"); 
    let do_print_nums = print_nums.to_lowercase() == "y".to_string();

    let start_time = SystemTime::now();
    while num > one.into() {
        if is_odd(&num) {
            num = num * three + one;
        } else {
            num = num / two;
        }
        if do_print_nums {
            println!("{}", num)
        }
    }
    
    println!("confirmed, done in {:#?}.", SystemTime::now().duration_since(start_time).unwrap());
    pause();
}
