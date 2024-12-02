use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open file: {err}");
            std::process::exit(1);
        }
    };

    let result = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            let ns = line
                .split(' ')
                .filter_map(|col| col.trim().parse::<i32>().ok())
                .collect::<Vec<_>>();

            if ns.len() == 0 {
                return false;
            }

            if ns.len() == 1 {
                return true;
            }

            (0..ns.len())
                .filter(|i| {
                    let mut ns = ns.clone();
                    ns.remove(*i);
                    let diffs = ns.windows(2).map(|p| p[0] - p[1]).collect::<Vec<_>>();
                    let is_asc = diffs.iter().all(|diff| *diff > 0);
                    let is_desc = diffs.iter().all(|diff| *diff < 0);
                    let is_in_range = !diffs
                        .iter()
                        .any(|diff| *diff < -3 || *diff > 3 || *diff == 0);
                    (is_asc || is_desc) && is_in_range
                })
                .next()
                .is_some()
        })
        .count();

    println!("{result}");
}
