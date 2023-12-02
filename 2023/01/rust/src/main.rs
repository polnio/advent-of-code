mod part1;
mod part2;

use std::env::{self, args};
use std::fs::File;

fn main() {
    let filename = args().skip(1).next().expect("no filename provided");
    let file = File::open(&filename).expect("failed to open file");

    if env::var("PART").unwrap_or("1".to_string()) == "1" {
        let sum = part1::get_result(&file);
        println!("{}", sum);
    } else {
        let sum = part2::get_result(&file);
        println!("{}", sum);
    }
}
