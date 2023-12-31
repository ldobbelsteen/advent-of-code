#![warn(clippy::pedantic)]

use anyhow::Result;
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Space {
    dimensions: Coord,
    galaxies: Vec<Coord>,
}

impl Space {
    fn expand(&mut self) {
        let mut empty_rows = HashSet::new();
        let mut empty_cols = HashSet::new();

        for row in 0..self.dimensions.y {
            if !self.galaxies.iter().any(|g| g.y == row) {
                empty_rows.insert(row);
            }
        }
        for col in 0..self.dimensions.x {
            if !self.galaxies.iter().any(|g| g.x == col) {
                empty_cols.insert(col);
            }
        }

        for galaxy in &mut self.galaxies {
            let y_expand = empty_rows.iter().filter(|row| **row < galaxy.y).count();
            let x_expand = empty_cols.iter().filter(|col| **col < galaxy.x).count();
            *galaxy = Coord {
                x: galaxy.x + x_expand,
                y: galaxy.y + y_expand,
            };
        }
    }
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut galaxies = Vec::new();
        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    galaxies.push(Coord { x, y });
                }
            });
        });
        Ok(Space {
            dimensions: Coord {
                x: galaxies.iter().map(|g| g.x).max().unwrap_or(0),
                y: galaxies.iter().map(|g| g.y).max().unwrap_or(0),
            },
            galaxies,
        })
    }
}

fn manhattan_distance(point1: &Coord, point2: &Coord) -> usize {
    (if point1.x > point2.x {
        point1.x - point2.x
    } else {
        point2.x - point1.x
    }) + (if point1.y > point2.y {
        point1.y - point2.y
    } else {
        point2.y - point1.y
    })
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut space = Space::from_str(&file)?;
    space.expand();

    let mut result = 0;
    for i in 0..space.galaxies.len() {
        for j in i + 1..space.galaxies.len() {
            result += manhattan_distance(&space.galaxies[i], &space.galaxies[j]);
        }
    }

    println!("{result}");
    Ok(())
}
