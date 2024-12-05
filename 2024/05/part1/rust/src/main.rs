use std::fs::File;
use std::io::{BufRead as _, BufReader};

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            std::process::exit(1);
        }
    };

    let mut lines = BufReader::new(file).lines().filter_map(Result::ok);
    let order_rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (a, b) = line.split_once('|')?;
            let a = a.parse::<u32>().ok()?;
            let b = b.parse::<u32>().ok()?;
            Some((a, b))
        })
        .collect::<Vec<_>>();

    let result = lines
        .filter_map(|line| {
            let numbers = line
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .try_fold(Vec::new(), |mut acc, n| {
                    if order_rules.iter().any(|(a, b)| *a == n && acc.contains(b)) {
                        return None;
                    }
                    acc.push(n);
                    Some(acc)
                })?;
            Some(numbers[numbers.len() / 2])
        })
        .sum::<u32>();

    println!("{result}");
}
