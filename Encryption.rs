use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'encryption' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn encryption(s: &str) -> String {
    
    let s: String = s.chars().filter(|&c| c != ' ').collect();
    let len = s.len();
    
    let rows = (len as f64).sqrt().floor() as usize;
    let cols = (len as f64).sqrt().ceil() as usize;
    let (rows, cols) = if rows * cols < len {
        (rows + 1, cols)
    } else {
        (rows, cols)
    };

    let mut result = Vec::new();
    for col in 0..cols {
        let mut temp = String::new();
        for row in 0..rows {
            let idx = row * cols + col;
            if idx < len {
                temp.push(s.chars().nth(idx).unwrap());
            }
        }
        result.push(temp);
    }

    result.join(" ")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = encryption(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
