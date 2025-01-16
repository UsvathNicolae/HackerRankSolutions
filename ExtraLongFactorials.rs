use std::io::{self, BufRead};
use num::BigUint;

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut result = BigUint::from(1u32);
    for i in 1..=n {
        result *= BigUint::from(i as u32);
    }
    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
