mod part1;
mod part2;

use std::env::{self, args};
use std::fs::File;

fn main() {
    let filename = args().nth(1).expect("No filename given");
    let file = File::open(filename).expect("File not found");
    let part = env::var("PART").unwrap_or("1".to_string());

    if part == "1" {
        println!("{}", part1::get_result(&file));
    } else {
        println!("{}", part2::get_result(&file));
    }
}
