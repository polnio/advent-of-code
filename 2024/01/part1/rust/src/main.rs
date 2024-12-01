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

    let (mut n1s, mut n2s): (Vec<_>, Vec<_>) = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            let (n1, n2) = line.split_once(' ')?;
            let n1 = n1.trim().parse::<u32>().ok()?;
            let n2 = n2.trim().parse::<u32>().ok()?;
            Some((n1, n2))
        })
        .unzip();

    n1s.sort_unstable();
    n2s.sort_unstable();

    let result = Iterator::zip(n1s.into_iter(), n2s.into_iter())
        .map(|(n1, n2)| u32::abs_diff(n1, n2))
        .sum::<u32>();

    println!("{result}");
}
