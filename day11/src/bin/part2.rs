use anyhow::Result;
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Space {
    dimensions: Coord,
    galaxies: Vec<Coord>,
}

impl Space {
    fn expand(&mut self) {
        let multiplier = 1_000_000;
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

        self.dimensions = Coord {
            x: self.dimensions.x + (multiplier - 1) * empty_rows.len() as i64,
            y: self.dimensions.y + (multiplier - 1) * empty_cols.len() as i64,
        };

        for galaxy in self.galaxies.iter_mut() {
            let y_expand =
                (multiplier - 1) * empty_rows.iter().filter(|row| **row < galaxy.y).count() as i64;
            let x_expand =
                (multiplier - 1) * empty_cols.iter().filter(|col| **col < galaxy.x).count() as i64;
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
                    galaxies.push(Coord {
                        x: x as i64,
                        y: y as i64,
                    })
                }
            })
        });
        Ok(Space {
            galaxies,
            dimensions: Coord {
                y: s.lines().count() as i64,
                x: s.lines().next().unwrap().chars().count() as i64,
            },
        })
    }
}

fn manhattan_distance(point1: &Coord, point2: &Coord) -> i64 {
    (point2.x - point1.x).abs() + (point2.y - point1.y).abs()
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

    println!("{:?}", result);
    Ok(())
}
