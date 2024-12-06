#[derive(Debug, Clone, Default, PartialEq, Eq)]
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
    Visited,
}
impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Guard,
            'X' => Self::Visited,
            _ => Self::default(),
        }
    }
}

struct Map(Vec<Vec<Point>>);
impl Map {
    fn get(&self, row: usize, col: usize) -> Option<&Point> {
        self.0.get(row).and_then(|row| row.get(col))
    }
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Point> {
        self.0.get_mut(row).and_then(|row| row.get_mut(col))
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
    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter().flatten()
    }
}

struct Guard {
    position: (usize, usize),
    direction: Direction,
}
impl Guard {
    fn try_walk(&mut self, map: &mut Map) -> bool {
        let (row, col) = self.position;
        map.get_mut(row, col).map(|point| *point = Point::Visited);
        let (new_row, new_col) = match self.direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };
        let Some(new_point) = map.get(new_row, new_col) else {
            return false;
        };
        match new_point {
            Point::Empty | Point::Visited => {
                self.position = (new_row, new_col);
            }
            Point::Wall => {
                self.direction = self.direction.turn();
            }
            Point::Guard => {
                eprintln!("Guard found at {new_row}:{new_col}");
                std::process::exit(0);
            }
        }
        true
    }
}

impl Map {
    fn load(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read_to_string(path)?;
        let data = data
            .lines()
            .map(|line| line.chars().map(Point::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Self(data))
    }
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <path>");
        std::process::exit(1);
    };
    let mut map = match Map::load(&path) {
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
    let mut guard = Guard {
        position,
        direction: Direction::Up,
    };
    loop {
        let still_in_map = guard.try_walk(&mut map);
        if !still_in_map {
            break;
        }
    }
    let count = map.iter().filter(|p| **p == Point::Visited).count();
    println!("{count}");
}
