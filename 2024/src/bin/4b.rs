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

    // Check whether the core of an x-word is at a coordinate.
    fn is_x_word_core(&self, coord: &Coord, word: [char; 3]) -> Result<bool> {
        // The core has to be the middle letter.
        if self.get(coord)? != Some(word[1]) {
            return Ok(false);
        }

        if let Some(top_left) = self.get(&Coord {
            x: coord.x - 1,
            y: coord.y - 1,
        })? {
            // The top left character should be the first or third letter.
            if top_left != word[0] && top_left != word[2] {
                return Ok(false);
            }

            if let Some(bottom_right) = self.get(&Coord {
                x: coord.x + 1,
                y: coord.y + 1,
            })? {
                // The bottom right character should be the first or third letter.
                if bottom_right != word[0] && bottom_right != word[2] {
                    return Ok(false);
                }

                // The top left and bottom right characters should not be the same.
                if top_left == bottom_right {
                    return Ok(false);
                }
            } else {
                // If we are on the edge, no x-word can be formed.
                return Ok(false);
            }
        } else {
            // If we are on the edge, no x-word can be formed.
            return Ok(false);
        }

        if let Some(top_right) = self.get(&Coord {
            x: coord.x + 1,
            y: coord.y - 1,
        })? {
            // The top right character should be the first or third letter.
            if top_right != word[0] && top_right != word[2] {
                return Ok(false);
            }

            if let Some(bottom_left) = self.get(&Coord {
                x: coord.x - 1,
                y: coord.y + 1,
            })? {
                // The bottom left character should be the first or third letter.
                if bottom_left != word[0] && bottom_left != word[2] {
                    return Ok(false);
                }

                // The top right and bottom left characters should not be the same.
                if top_right == bottom_left {
                    return Ok(false);
                }
            } else {
                // If we are on the edge, no x-word can be formed.
                return Ok(false);
            }
        } else {
            // If we are on the edge, no x-word can be formed.
            return Ok(false);
        }

        Ok(true)
    }

    // Count the occurrences of an x-word in the grid.
    fn count_x_words(&self, word: [char; 3]) -> Result<i32> {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord {
                    x: i32::try_from(x)?,
                    y: i32::try_from(y)?,
                };

                if self.is_x_word_core(&coord, word)? {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/4-input.txt")?;
    let grid = Grid::from_str(&file)?;

    let result = grid.count_x_words(['M', 'A', 'S'])?;
    println!("Result: {result}");

    Ok(())
}
