use std::fs::File;
use std::io::{BufRead, BufReader};

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn get_result(file: &File) -> u32 {
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        for i in 0..line.len() {
            let rest = &line[i..];
            let mut found = false;
            for j in 0..NUMBERS.len() {
                if rest.starts_with(NUMBERS[j]) {
                    if first_digit.is_none() {
                        first_digit = Some(j as u32);
                    } else {
                        last_digit = Some(j as u32);
                    }
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            let first_char = rest.chars().next();
            if let Some(c) = first_char {
                if c.is_digit(10) {
                    let digit = c.to_digit(10).unwrap();
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    } else {
                        last_digit = Some(digit);
                    }
                }
            }
        }

        if first_digit.is_none() {
            continue;
        }
        let first_digit = first_digit.unwrap();
        let last_digit = last_digit.unwrap_or(first_digit);
        sum += first_digit * 10 + last_digit;
    }

    sum
}
