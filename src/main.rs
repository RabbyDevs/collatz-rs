// use std::intrinsics::sqrtf64;
use std::io;
use std::str::FromStr;
use num::bigint::BigUint;
use num::FromPrimitive;
use io::{stdout, stdin, Read, Write};

fn new_parameter(ask_string: &str) -> String {
    // clear_terminal_screen();
    println!("{}", ask_string);
    let mut parameter = String::new();
    
    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to process input.");
    parameter
}

fn is_odd(num: &BigUint, one: &BigUint, two: &BigUint) -> bool {
    let num = num.modpow(one, two);
    if num == BigUint::from_u16(0).unwrap() {
        return false
    } else {
        return true
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("Collatz thing!");
    let two = BigUint::from_u16(2).unwrap();
    let three = BigUint::from_u16(3).unwrap();
    let one = BigUint::from_u16(1).unwrap();
    let mut num: BigUint = BigUint::from_str(new_parameter("What number do you want to calculate? ").as_str().trim_end()).unwrap();
    let mut done = false;
    while done == false {
        let is_odd = is_odd(&num, &one, &two);
        if is_odd == false {
            num = num / &two;
            println!("{:#?}", num);
            if num.to_string() == "1".to_string() {done = true}
        } else {
            num = num * &three;
            num = num + &one;
            println!("{:#?}", num);
            if num.to_string() == "1".to_string() {done = true}
        }
    }
    println!("confirmed.");
    pause()
}