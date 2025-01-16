use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let hour: i32 = s[0..2].parse().unwrap();
    let minute = &s[3..5];
    let second = &s[6..8];
    let period = &s[8..];

    let military_hour = match period {
        "AM" if hour == 12 => "00".to_string(),
        "AM" => format!("{:02}", hour),
        "PM" if hour == 12 => "12".to_string(),
        "PM" => format!("{:02}", hour + 12),
        _ => panic!("Invalid time format"),
    };

    format!("{}:{}:{}", military_hour, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
