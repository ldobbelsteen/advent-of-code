#![warn(clippy::pedantic)]

use anyhow::Result;
use smallvec::SmallVec;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn neighboring_coords(c: &Coord, width: usize, height: usize) -> SmallVec<[Coord; 4]> {
    let mut coords = SmallVec::new();

    if c.x > 0 {
        coords.push(Coord { x: c.x - 1, y: c.y });
    }

    if c.x < width - 1 {
        coords.push(Coord { x: c.x + 1, y: c.y });
    }

    if c.y > 0 {
        coords.push(Coord { x: c.x, y: c.y - 1 });
    }

    if c.y < height - 1 {
        coords.push(Coord { x: c.x, y: c.y + 1 });
    }

    coords
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    plots: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Area {
    size: usize,
    perimeter: usize,
}

impl Area {
    fn fencing_cost(&self) -> usize {
        self.size * self.perimeter
    }
}

impl Map {
    fn from_str(s: &str) -> Self {
        let plots = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = plots.len();
        let width = if plots.is_empty() { 0 } else { plots[0].len() };

        Self {
            width,
            height,
            plots,
        }
    }

    fn fencing_cost(&self) -> usize {
        let mut result = 0;
        let mut visited = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                if !visited[y][x] {
                    let area = self.compute_area(&Coord { x, y }, &mut visited);
                    result += area.fencing_cost();
                }
            }
        }

        result
    }

    // Floodfill from the current coordinate and compute the resulting area with
    // the given label. Marks visited coordinates in the visited matrix.
    fn compute_area(&self, coord: &Coord, visited: &mut [Vec<bool>]) -> Area {
        debug_assert!(!visited[coord.y][coord.x]);

        let label = self.plots[coord.y][coord.x];
        let mut queue = VecDeque::from([*coord]);

        let mut area = Area {
            size: 0,
            perimeter: 0,
        };

        while let Some(coord) = queue.pop_front() {
            if visited[coord.y][coord.x] {
                continue;
            }

            visited[coord.y][coord.x] = true;
            area.perimeter += 4;
            area.size += 1;

            let neighbors = neighboring_coords(&coord, self.width, self.height);
            for neighbor in neighbors {
                if self.plots[neighbor.y][neighbor.x] == label {
                    queue.push_back(neighbor);
                    area.perimeter -= 1;
                }
            }
        }

        area
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/12-input.txt")?;
    let map = Map::from_str(&file);

    let result = map.fencing_cost();
    println!("result: {result}");

    Ok(())
}
