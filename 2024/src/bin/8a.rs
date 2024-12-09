#![warn(clippy::pedantic)]

use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Coord>>,
}

impl Map {
    fn from_str(s: &str) -> Result<Self> {
        let mut width = None;
        let mut height = 0;

        let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            height += 1;
            if let Some(w) = width {
                if w != line.len() {
                    return Err(anyhow::anyhow!("inconsistent line length"));
                }
            } else {
                width = Some(line.len());
            }

            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_default().push(Coord {
                        x: x.try_into()?,
                        y: y.try_into()?,
                    });
                }
            }
        }

        Ok(Self {
            width: width.unwrap_or(0),
            height,
            antennas,
        })
    }

    fn within_bounds(&self, coord: Coord) -> Result<bool> {
        if coord.x >= 0 && coord.y >= 0 {
            let x: usize = coord.x.try_into()?;
            let y: usize = coord.y.try_into()?;
            if x < self.width && y < self.height {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Get the two antinodes caused by two antennas of the same kind. Does not
    /// check if the antinodes are within bounds.
    fn get_antinode_pair(a: Coord, b: Coord) -> (Coord, Coord) {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        let first = Coord {
            x: b.x + dx,
            y: b.y + dy,
        };

        let second = Coord {
            x: a.x - dx,
            y: a.y - dy,
        };

        (first, second)
    }

    /// Get all antinodes caused by any pair of antennas of the same kind. The
    /// coords are all within bounds and returned as a set.
    fn get_all_antinodes(&self) -> Result<HashSet<Coord>> {
        let mut result = HashSet::new();

        for coords in self.antennas.values() {
            for (a, b) in coords.iter().tuple_combinations() {
                let (first, second) = Self::get_antinode_pair(*a, *b);
                if self.within_bounds(first)? {
                    result.insert(first);
                }
                if self.within_bounds(second)? {
                    result.insert(second);
                }
            }
        }

        Ok(result)
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/8-input.txt")?;
    let map = Map::from_str(&file)?;
    let antinodes = map.get_all_antinodes()?;

    let result = antinodes.len();
    println!("result: {result}");

    Ok(())
}
