use std::collections::LinkedList;
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
            let (numbers, was_invalid) = line
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .fold((LinkedList::new(), false), |(mut acc, was_invalid), n| {
                    let rules = order_rules
                        .iter()
                        .filter(|(a, b)| *a == n && acc.contains(b))
                        .copied()
                        .collect::<Vec<_>>();
                    if rules.is_empty() {
                        acc.push_back(n);
                        (acc, was_invalid)
                    } else {
                        let index = acc
                            .iter()
                            .position(|&n| rules.iter().any(|(_, x)| *x == n))
                            .unwrap_or_default();
                        let mut tail = acc.split_off(index);
                        acc.push_back(n);
                        acc.append(&mut tail);
                        (acc, true)
                    }
                });
            if !was_invalid {
                return None;
            }
            let len = numbers.len();
            numbers.into_iter().nth(len / 2)
        })
        .sum::<u32>();

    println!("{result}");
}
