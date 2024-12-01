use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("Usage: cargo run -- <input-file>");
        std::process::exit(1);
    };

    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file: {}", error);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    let (left, right): (Vec<_>, Vec<_>) = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let (n1, n2) = line.split_once(' ')?;
            let n1 = n1.trim().parse::<u32>().ok()?;
            let n2 = n2.trim().parse::<u32>().ok()?;
            Some((n1, n2))
        })
        .unzip();

    let counts = right.into_iter().fold(HashMap::new(), |mut counts, n| {
        *counts.entry(n).or_insert(0) += 1;
        counts
    });

    let result = left
        .into_iter()
        .map(|n| n * counts.get(&n).unwrap_or(&0))
        .sum::<u32>();

    println!("{result}");
}
