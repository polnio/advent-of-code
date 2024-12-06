use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Point {
    #[default]
    Empty,
    Wall,
    Guard,
}
impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Guard,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Map(Vec<Vec<Point>>);
impl Map {
    fn load(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read_to_string(path)?;
        let data = data
            .lines()
            .map(|line| line.chars().map(Point::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self(data))
    }
    fn get(&self, row: usize, col: usize) -> Option<&Point> {
        self.0.get(row).and_then(|row| row.get(col))
    }
    fn find(&self, point: Point) -> Option<(usize, usize)> {
        self.0.iter().enumerate().find_map(|(row, row_points)| {
            row_points.iter().enumerate().find_map(|(col, col_point)| {
                if *col_point == point {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
    }
    #[allow(unused)]
    fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &Point)> {
        self.0.iter().enumerate().flat_map(|(row, row_points)| {
            row_points
                .iter()
                .enumerate()
                .map(move |(col, col_point)| (row, col, col_point))
        })
    }
    #[allow(unused)]
    fn par_enumerate(&self) -> impl ParallelIterator<Item = (usize, usize, &Point)> {
        self.0.par_iter().enumerate().flat_map(|(row, row_points)| {
            row_points
                .par_iter()
                .enumerate()
                .map(move |(col, col_point)| (row, col, col_point))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WalkResult {
    Continue,
    Outside,
    Loop,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}
impl Guard {
    fn walk(
        &mut self,
        map: &Map,
        fake_wall: (usize, usize),
        relevant_visited: &mut HashMap<(usize, usize), HashSet<Direction>>,
    ) -> WalkResult {
        let (row, col) = self.position;
        let (new_row, new_col) = match self.direction {
            Direction::Up => (row.wrapping_sub(1), col),
            Direction::Down => (row.wrapping_add(1), col),
            Direction::Left => (row, col.wrapping_sub(1)),
            Direction::Right => (row, col.wrapping_add(1)),
        };
        let Some(new_point) = map.get(new_row, new_col) else {
            return WalkResult::Outside;
        };
        if matches!(new_point, Point::Wall) || fake_wall == (new_row, new_col) {
            let visited = relevant_visited
                .entry((row, col))
                .or_insert_with(HashSet::new);
            if visited.contains(&self.direction) {
                return WalkResult::Loop;
            }
            visited.insert(self.direction.clone());
            self.direction = self.direction.turn();
        } else {
            self.position = (new_row, new_col);
        }
        WalkResult::Continue
    }
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };
    let map = match Map::load(&path) {
        Ok(map) => map,
        Err(err) => {
            eprintln!("Error loading map: {err}");
            std::process::exit(1);
        }
    };
    let Some(position) = map.find(Point::Guard) else {
        eprintln!("No guard found");
        std::process::exit(1);
    };
    let guard = Guard {
        position,
        direction: Direction::Up,
    };
    let count = map
        // .enumerate()
        .par_enumerate()
        .filter_map(|(row, col, _)| {
            // println!("{row}:{col}");
            if row == guard.position.0 && col == guard.position.1 {
                return None;
            }
            // DEBUG
            let mut guard = guard.clone();
            let mut relevant_visited = HashMap::new();
            let result = loop {
                match guard.walk(&map, (row, col), &mut relevant_visited) {
                    WalkResult::Continue => {}
                    WalkResult::Outside => break false,
                    WalkResult::Loop => break true,
                }
            };
            result.then_some(())
        })
        .count();
    println!("{count}");
}
