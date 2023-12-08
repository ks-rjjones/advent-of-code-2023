use core::panic;
use std::{fs::File, io::Read, time::Instant};

// runtime: 631µs
// Run with 'cargo run --bin day1_2'
fn main() {
    let now = Instant::now();

    let mut file = File::open("inputs/day1_2.txt").expect("File not found!");
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

fn word_to_digit(str: &str) -> &str {
    match str {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "0",
    }
}

fn first_and_last_digit(str: &str) -> u8 {
    let mut first_digit = None;
    let mut last_digit = None;

    let mut str_buffer = String::new();
    for mut c in str.chars() {
        if c.is_numeric() {
            if !str_buffer.is_empty() {
                let digit = word_to_digit(&str_buffer);
                c = digit.chars().next().expect("Could not get first char");
                str_buffer.clear();
            }

            last_digit = Some(c);
            if first_digit.is_none() {
                first_digit = Some(c);
            }
        } else {
            str_buffer.push(c);
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            let number_str = format!("{}{}", first, last);
            number_str.parse::<u8>().expect("Could not parse number")
        }
        (Some(first), None) => {
            // If there is one digit, duplicate to get the number
            let number_str = format!("{}{}", first, first);
            number_str.parse::<u8>().expect("THIS SHOULD NEVER HAPPEN!")
        }
        _ => panic!("THIS SHOULD NEVER HAPPEN!"),
    }
}

// Run with 'cargo test --bin day1_2' or 'cargo test test_day1_2'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_2_1() {
        let test_cases = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let results = vec![29, 83, 13, 24, 42, 14, 76];

        for (index, str) in test_cases.iter().enumerate() {
            let result = first_and_last_digit(str);
            let expected = results[index];
            assert_eq!(result, expected, "Expected {result} to be {expected}")
        }
    }
}
