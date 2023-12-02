use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_result(file: &File) -> u32 {
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        let mut chars = line.chars();
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        while let Some(c) = chars.next() {
            if !c.is_digit(10) {
                continue;
            }
            let digit = c.to_digit(10).unwrap();
            if first_digit.is_none() {
                first_digit = Some(digit);
            } else {
                last_digit = Some(digit);
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
