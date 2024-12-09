#![warn(clippy::pedantic)]

use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoordVec {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<CoordVec>>,
}

impl Map {
    fn from_str(s: &str) -> Result<Self> {
        let mut width = None;
        let mut height = 0;

        let mut antennas: HashMap<char, Vec<CoordVec>> = HashMap::new();
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
                    antennas.entry(c).or_default().push(CoordVec {
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

    fn within_bounds(&self, coord: CoordVec) -> bool {
        if coord.x < 0 || coord.y < 0 {
            return false;
        }

        let x: usize = coord.x.try_into().unwrap();
        let y: usize = coord.y.try_into().unwrap();
        x < self.width && y < self.height
    }

    /// Get the antinodes caused by two antennas of the same kind. The coords
    /// returned are all guaranteed to be within bounds.
    fn get_antinodes_for_pair(
        &'_ self,
        a: CoordVec,
        b: CoordVec,
    ) -> impl Iterator<Item = CoordVec> + '_ {
        let d = CoordVec {
            x: b.x - a.x,
            y: b.y - a.y,
        };

        let forward = (0..)
            .map(move |n| CoordVec {
                x: b.x + d.x * n,
                y: b.y + d.y * n,
            })
            .take_while(|c| self.within_bounds(*c));

        let backward = (0..)
            .map(move |n| CoordVec {
                x: a.x - d.x * n,
                y: a.y - d.y * n,
            })
            .take_while(|c| self.within_bounds(*c));

        forward.chain(backward)
    }

    /// Get all antinodes caused by any pair of antennas of the same kind. The
    /// coords are all within bounds and returned as a set.
    fn get_all_antinodes(&self) -> HashSet<CoordVec> {
        let mut result = HashSet::new();

        for coords in self.antennas.values() {
            for (a, b) in coords.iter().tuple_combinations() {
                for antinode in self.get_antinodes_for_pair(*a, *b) {
                    if self.within_bounds(antinode) {
                        result.insert(antinode);
                    }
                }
            }
        }

        result
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/8-input.txt")?;
    let map = Map::from_str(&file)?;
    let antinodes = map.get_all_antinodes();

    let result = antinodes.len();
    println!("result: {result}");

    Ok(())
}
