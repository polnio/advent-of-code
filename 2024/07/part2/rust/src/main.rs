use std::fs::File;
use std::io::{BufRead as _, BufReader};

fn test_equation(t: u64, c: u64, ns: &[u64]) -> bool {
    if ns.len() == 0 {
        return t == c;
    }
    let n = ns[0];
    let nc = format!("{c}{n}").parse::<u64>().unwrap();
    return test_equation(t, c + n, &ns[1..])
        || test_equation(t, c * n, &ns[1..])
        || test_equation(t, nc, &ns[1..]);
}

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

    let result = BufReader::new(file)
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let (t, ns) = line.split_once(':')?;
            let t = t.parse::<u64>().ok()?;
            let ns = ns
                .split_whitespace()
                .map(|s| s.parse::<u64>().ok())
                .collect::<Option<Vec<_>>>()?;
            test_equation(t, 0, &ns).then_some(t)
        })
        .sum::<u64>();

    println!("{result}");
}
