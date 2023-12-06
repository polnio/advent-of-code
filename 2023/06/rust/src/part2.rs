use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

pub fn get_result(file: &File) -> u64 {
    let times_regex: Regex = Regex::new(r"Time:((?:\s+\d+)+).*").unwrap();
    let distances_regex: Regex = Regex::new(r"Distance:((?:\s+\d+)+).*").unwrap();

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let times_line = lines.next().unwrap().unwrap();
    let distances_line = lines.next().unwrap().unwrap();

    let time = times_regex
        .captures(&times_line)
        .unwrap()
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let distance = distances_regex
        .captures(&distances_line)
        .unwrap()
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let mut result = 1;

    let mut i = 0;
    while i < time && (time - i) * i <= distance {
        i += 1;
    }
    // + 1 because i starts at 0, and 2 * i because symmetric
    let count = time + 1 - 2 * i;
    result *= count;

    result
}
