use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut sum: i64 = 0; 
    let min = arr.iter().min().unwrap(); 
    let max = arr.iter().max().unwrap(); 
    
    for i in arr{
        sum += *i as i64;
    }

    let min_sum:i64 = (sum as i64) - (*max as i64);
    let max_sum:i64 = (sum as i64) - (*min as i64);

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
