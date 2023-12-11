#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Bounds {
    x_min: usize,
    x_max: usize, // exclusive
    y_min: usize,
    y_max: usize, // exclusive
}

impl Bounds {
    fn within(&self, coords: &Coord) -> bool {
        coords.x >= self.x_min
            && coords.x < self.x_max
            && coords.y >= self.y_min
            && coords.y < self.y_max
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    E,
    W,
    N,
    S,
}

impl Direction {
    /// Move in this direction for a specific coordinate.
    fn take(&self, coord: &Coord) -> Coord {
        match self {
            Self::E => Coord {
                x: coord.x + 1,
                y: coord.y,
            },
            Self::W => Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            Self::N => Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            Self::S => Coord {
                x: coord.x,
                y: coord.y + 1,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Connections {
    incoming: Direction,
    outgoing: Direction,
}

#[derive(Debug, PartialEq)]
enum Tile {
    NorthToSouth,
    WestToEast,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Start,
    Ground,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '|' => Ok(Self::NorthToSouth),
            '-' => Ok(Self::WestToEast),
            'L' => Ok(Self::NorthToEast),
            'J' => Ok(Self::NorthToWest),
            '7' => Ok(Self::SouthToWest),
            'F' => Ok(Self::SouthToEast),
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            _ => Err(anyhow!("char is not a valid tile: {}", c)),
        }
    }

    /// Get the endpoints of the pipes on this tile.
    fn endpoints(&self) -> HashSet<Direction> {
        match self {
            Self::NorthToSouth => HashSet::from([Direction::N, Direction::S]),
            Self::WestToEast => HashSet::from([Direction::W, Direction::E]),
            Self::NorthToEast => HashSet::from([Direction::N, Direction::E]),
            Self::NorthToWest => HashSet::from([Direction::N, Direction::W]),
            Self::SouthToWest => HashSet::from([Direction::S, Direction::W]),
            Self::SouthToEast => HashSet::from([Direction::S, Direction::E]),
            Self::Start => HashSet::from([Direction::N, Direction::S, Direction::W, Direction::E]),
            Self::Ground => HashSet::from([]),
        }
    }
}

#[derive(Debug)]
struct Grid {
    start: Coord,
    tiles: Vec<Vec<Tile>>,
    bounds: Bounds,
}

impl Grid {
    /// Get the adjacent coordinates to a pipe at a coordinate. Only returns coordinates
    /// actually within the bounds and actually at endpoints of the pipe.
    fn adjacent(&self, coord: &Coord) -> HashSet<Coord> {
        self.tiles[coord.y][coord.x]
            .endpoints()
            .into_iter()
            .map(|d| d.take(coord))
            .filter(|c| self.bounds.within(c))
            .collect()
    }

    /// Get the adjacent coordinates which have tiles where the endpoints match
    /// e.g. they can connect and form a cycle.
    fn matching_adjacent(&self, coord: &Coord) -> HashSet<Coord> {
        self.adjacent(coord)
            .into_iter()
            .filter(|c| self.adjacent(c).contains(coord))
            .collect()
    }

    /// Find a loop/cycle of pipes starting from the grid's starting point.
    fn find_cycle(&self) -> Result<Vec<Coord>> {
        fn depth_first_search(
            grid: &Grid,
            previous: &Option<Coord>,
            current: &Coord,
            path: &mut Vec<Coord>,
        ) -> bool {
            path.push(current.clone());
            for next in grid.matching_adjacent(current) {
                if let Some(previous) = &previous {
                    if next == *previous {
                        continue; // circling back of not valid
                    }
                }
                if next == grid.start {
                    return true; // we have come back to the start and are finished
                }
                if depth_first_search(grid, &Some(current.clone()), &next, path) {
                    return true; // if this branch succeeded down the line, we are finished
                }
            }
            path.pop(); // none of the branches succeeded, thus remove the element again
            false
        }

        let mut result = Vec::new();
        if depth_first_search(self, &None, &self.start, &mut result) {
            Ok(result)
        } else {
            Err(anyhow!("no cycle could be found in grid: {:?}", self))
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::from_char)
                    .collect::<Result<Vec<Tile>>>()
            })
            .collect::<Result<Vec<Vec<Tile>>>>()?;
        let start_row = tiles
            .iter()
            .position(|row| row.contains(&Tile::Start))
            .ok_or(anyhow!("no starting point found in grid: {}", s))?;
        let start_col = tiles[start_row]
            .iter()
            .position(|t| *t == Tile::Start)
            .unwrap();
        Ok(Grid {
            start: Coord {
                x: start_col,
                y: start_row,
            },
            bounds: Bounds {
                x_min: 0,
                x_max: tiles[0].len(),
                y_min: 0,
                y_max: tiles.len(),
            },
            tiles,
        })
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let grid = Grid::from_str(&file)?;
    let cycle = grid.find_cycle()?;
    println!("{}", cycle.len() / 2);
    Ok(())
}
