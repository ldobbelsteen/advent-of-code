#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    inner: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(input: &str) -> Result<Self> {
        let inner: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = inner.len();
        let width = if height == 0 { 0 } else { inner[0].len() };

        for row in &inner {
            if row.len() != width {
                return Err(anyhow!("grid is not rectangular"));
            }
        }

        Ok(Self {
            inner,
            width,
            height,
        })
    }

    fn get(&self, coord: &Coord) -> Result<Option<char>> {
        if coord.x < 0 || coord.x >= i32::try_from(self.width)? {
            return Ok(None);
        }

        if coord.y < 0 || coord.y >= i32::try_from(self.height)? {
            return Ok(None);
        }

        Ok(Some(
            self.inner[usize::try_from(coord.y)?][usize::try_from(coord.x)?],
        ))
    }

    // Count the occurrences of a word starting at a coordinate.
    // Looks in all 8 directions.
    fn count_word_occs_at(&self, coord: &Coord, word: &str) -> Result<i32> {
        let mut count = 0;

        // For x and y, check all combinations of negative, positive, and neutral directions.
        for dy in -1..=1 {
            for dx in -1..=1 {
                // Skip the case where we don't move since both are neutral.
                if dx == 0 && dy == 0 {
                    continue;
                }

                let mut found = true;
                for (i, c) in word.chars().enumerate() {
                    let coord = Coord {
                        x: coord.x + i32::try_from(i)? * dx,
                        y: coord.y + i32::try_from(i)? * dy,
                    };

                    if let Some(grid_c) = self.get(&coord)? {
                        if grid_c != c {
                            found = false;
                            break;
                        }
                    } else {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    // Count the occurrences of a word in the grid. Occurrences can be
    // horizontal, vertical, diagonal, and backwards.
    fn count_word_occs(&self, word: &str) -> Result<i32> {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord {
                    x: i32::try_from(x)?,
                    y: i32::try_from(y)?,
                };
                count += self.count_word_occs_at(&coord, word)?;
            }
        }

        Ok(count)
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/4-input.txt")?;
    let grid = Grid::from_str(&file)?;

    let result = grid.count_word_occs("XMAS")?;
    println!("Result: {result}");

    Ok(())
}
