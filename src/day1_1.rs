use core::panic;
use std::{fs::File, io::Read, time::Instant};

// runtime: 340µs
// Run with 'cargo run --bin day1_1'
fn main() {
    let now = Instant::now();

    let mut file = File::open("inputs/day1_1.txt").expect("File not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file!");

    let mut sum: u128 = 0;
    for line in contents.lines() {
        sum += first_and_last_digit(line) as u128;
    }

    let elapsed = now.elapsed();
    println!("Sum: {}", sum);
    println!("Finished in {:.2?}", elapsed);
}

fn first_and_last_digit(str: &str) -> u8 {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in str.chars() {
        if c.is_numeric() {
            last_digit = Some(c);
            if first_digit.is_none() {
                first_digit = Some(c);
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            let number_str = format!("{}{}", first, last);
            number_str.parse::<u8>().expect("THIS SHOULD NEVER HAPPEN!")
        }
        (Some(first), None) => {
            // If there is one digit, duplicate to get the number
            let number_str = format!("{}{}", first, first);
            number_str.parse::<u8>().expect("THIS SHOULD NEVER HAPPEN!")
        }
        _ => panic!("THIS SHOULD NEVER HAPPEN!"),
    }
}

// Run with 'cargo test --bin day1_1' or 'cargo test test_day1_1'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1_1() {
        let test_cases = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let results = vec![12, 38, 15, 77];

        for (index, str) in test_cases.iter().enumerate() {
            let result = first_and_last_digit(str);
            let expected = results[index];
            assert_eq!(result, expected, "Expected {result} to be {expected}")
        }
    }
}
