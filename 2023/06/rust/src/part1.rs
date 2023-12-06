use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

pub fn get_result(file: &File) -> u32 {
    let times_regex: Regex = Regex::new(r"Time:((?:\s+\d+)+).*").unwrap();
    let distances_regex: Regex = Regex::new(r"Distance:((?:\s+\d+)+).*").unwrap();

    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let times_line = lines.next().unwrap().unwrap();
    let distances_line = lines.next().unwrap().unwrap();

    let time_strs = times_regex
        .captures(&times_line)
        .unwrap()
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("")
        .split_whitespace();

    let distance_strs = distances_regex
        .captures(&distances_line)
        .unwrap()
        .get(1)
        .map(|m| m.as_str())
        .unwrap_or("")
        .split_whitespace();

    let mut result = 1;

    for (time_str, distance_str) in time_strs.zip(distance_strs) {
        let time = time_str.parse::<u32>().unwrap();
        let distance = distance_str.parse::<u32>().unwrap();
        let mut i = 0;
        while i < time && (time - i) * i <= distance {
            i += 1;
        }
        // + 1 because i starts at 0, and 2 * i because symmetric
        let count = time + 1 - 2 * i;
        result *= count;
    }

    result
}
