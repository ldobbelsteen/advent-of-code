#![warn(clippy::pedantic)]

use anyhow::Result;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    fn perpendicular(self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn move_in(&self, direction: Direction, width: usize, height: usize) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < height - 1 {
                    Some(Self {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < width - 1 {
                    Some(Self {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }
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
    sides: usize,
}

impl Area {
    fn fencing_cost(&self) -> usize {
        self.size * self.sides
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
        let mut handled = HashSet::<(Coord, Direction)>::new();

        let mut area = Area { size: 0, sides: 0 };

        while let Some(coord) = queue.pop_front() {
            if visited[coord.y][coord.x] {
                continue;
            }

            visited[coord.y][coord.x] = true;
            area.size += 1;

            for d in &DIRECTIONS {
                if handled.contains(&(coord, *d)) {
                    continue;
                }

                if let Some(neighbor) = coord.move_in(*d, self.width, self.height) {
                    if self.plots[neighbor.y][neighbor.x] == label {
                        queue.push_back(neighbor);
                        continue;
                    }
                }

                area.sides += 1;

                // Since we want to compute sides, we need to mark the neighbors
                // perpendicular to the current direction. Keep going as long as
                // they also have a fence in the same direction. Mark the direction
                // for those neighbors as handled to avoid double counting.
                for p in d.perpendicular() {
                    let mut previous = coord;
                    while let Some(current) = previous.move_in(p, self.width, self.height) {
                        if self.plots[current.y][current.x] != label {
                            break;
                        }

                        if let Some(double_neighbor) = current.move_in(*d, self.width, self.height)
                        {
                            if self.plots[double_neighbor.y][double_neighbor.x] == label {
                                break;
                            }
                        }

                        handled.insert((current, *d));
                        previous = current;
                    }
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
