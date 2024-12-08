use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead as _, BufReader};
use std::num::NonZeroU8;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Tile {
    Antenna(NonZeroU8),
    #[default]
    Empty,
}
impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            0 => Tile::Empty,
            n => unsafe { Tile::Antenna(NonZeroU8::new_unchecked(n)) },
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Map(Vec<Vec<Tile>>);
impl Map {
    fn load(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let data = BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|&b| if b == b'.' { 0 } else { b })
                    .map(Tile::from)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(Self(data))
    }

    fn get(&self, row: usize, col: usize) -> Option<&Tile> {
        self.0.get(row).and_then(|row| row.get(col))
    }

    fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &Tile)> {
        self.0.iter().enumerate().flat_map(|(row, row_tiles)| {
            row_tiles
                .iter()
                .enumerate()
                .map(move |(col, tile)| (row, col, tile))
        })
    }
}

fn inc_iter<T, F>(mut f: F) -> impl Iterator<Item = T>
where
    F: FnMut(u32) -> Option<T>,
{
    let mut i = 0;
    std::iter::from_fn(move || {
        let res = f(i);
        i += 1;
        res
    })
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };

    let map = match Map::load(&path) {
        Ok(map) => map,
        Err(err) => {
            println!("Error loading map: {err}");
            std::process::exit(1);
        }
    };
    let result = map
        .enumerate()
        .filter_map(|(row, col, tile)| match tile {
            Tile::Antenna(f) => Some((row, col, f)),
            _ => None,
        })
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (row, col, f)| {
            acc.entry(f.get()).or_default().push((row, col));
            acc
        })
        .into_values()
        .fold(HashSet::new(), |mut acc, p| {
            for (pos1, pos2) in p.iter().tuple_combinations() {
                let diff = (
                    pos2.0 as isize - pos1.0 as isize,
                    pos2.1 as isize - pos1.1 as isize,
                );
                let n1s = inc_iter(|i| {
                    let i = i as isize;
                    let n = (
                        (pos1.0 as isize - i * diff.0) as usize,
                        (pos1.1 as isize - i * diff.1) as usize,
                    );
                    map.get(n.0, n.1).map(|_| n)
                });
                let n2s = inc_iter(|i| {
                    let i = i as isize;
                    let n = (
                        (pos2.0 as isize + i * diff.0) as usize,
                        (pos2.1 as isize + i * diff.1) as usize,
                    );
                    map.get(n.0, n.1).map(|_| n)
                });
                for an in itertools::chain(n1s, n2s) {
                    acc.insert(an);
                }
            }
            acc
        })
        .len();
    println!("{result:?}");
}
