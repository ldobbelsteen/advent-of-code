#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/10-input.txt")?;

    let map = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).ok_or(anyhow!("not a digit")))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;

    let height = map.len();
    let width = if map.is_empty() { 0 } else { map[0].len() };

    let neighbours = |Coord { x, y }: Coord| {
        ([
            if x > 0 {
                Some(Coord { x: x - 1, y })
            } else {
                None
            },
            if y > 0 {
                Some(Coord { x, y: y - 1 })
            } else {
                None
            },
            if x + 1 < width {
                Some(Coord { x: x + 1, y })
            } else {
                None
            },
            if y + 1 < height {
                Some(Coord { x, y: y + 1 })
            } else {
                None
            },
        ])
        .into_iter()
        .flatten()
    };

    // Run a search from a trailhead to find all nines reachable from it.
    // Makes sure to count nines that are reachable from multiple walks only once.
    let score = |trailhead: Coord| -> usize {
        let mut reachable_nines = HashSet::new();
        let mut queue = Vec::from([(trailhead, 0)]);

        while let Some((coord, height)) = queue.pop() {
            for neighbour in neighbours(coord) {
                let neighbour_height = map[neighbour.y][neighbour.x];
                if neighbour_height == height + 1 {
                    if neighbour_height == 9 {
                        reachable_nines.insert(neighbour);
                    } else {
                        queue.push((neighbour, height + 1));
                    }
                }
            }
        }

        reachable_nines.len()
    };

    let trailheads = map.iter().enumerate().flat_map(|(y, row)| {
        row.iter().enumerate().filter_map(move |(x, &cell)| {
            if cell == 0 {
                Some(Coord { x, y })
            } else {
                None
            }
        })
    });

    let result: usize = trailheads.map(score).sum();
    println!("result: {result}");

    Ok(())
}
