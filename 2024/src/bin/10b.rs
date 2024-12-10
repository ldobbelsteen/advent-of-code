#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashMap;

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

    // Run a BFS from a trailhead. We can count the number of ways to reach a
    // coordinate by summing the number of ways to reach its incoming neighbours.
    let rating = |trailhead: Coord| -> usize {
        let mut queue = HashMap::new();
        queue.insert(trailhead, 1);

        for height in 1..=9 {
            let prev = std::mem::take(&mut queue);
            for (coord, rating) in prev {
                for neighbour in neighbours(coord) {
                    if map[neighbour.y][neighbour.x] == height {
                        *queue.entry(neighbour).or_insert(0) += rating;
                    }
                }
            }
        }

        queue.into_values().sum()
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

    let result: usize = trailheads.map(rating).sum();
    println!("result: {result}");

    Ok(())
}
