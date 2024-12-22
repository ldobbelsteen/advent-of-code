#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn to_gps(self) -> usize {
        100 * self.y + self.x
    }

    fn take_step(self, d: Direction) -> Self {
        match d {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(anyhow!("invalid direction char: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("invalid tile char: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    robot: Coord,
}

impl Grid {
    fn from_str(s: &str) -> Result<Self> {
        let mut robot = None;

        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '@' {
                            if robot.is_some() {
                                Err(anyhow!("multiple robots found"))
                            } else {
                                robot = Some(Coord { x, y });
                                Ok(Tile::Empty)
                            }
                        } else {
                            Tile::from_char(c)
                        }
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            tiles,
            robot: robot.ok_or(anyhow!("no robot found"))?,
        })
    }

    fn get_tile(&self, c: Coord) -> Option<Tile> {
        self.tiles.get(c.y)?.get(c.x).copied()
    }

    fn move_box(&mut self, c: Coord, d: Direction) -> Result<()> {
        debug_assert!(matches!(self.get_tile(c), Some(Tile::Box)));
        let next_c = c.take_step(d);

        // Try to move any blocking box in front.
        if self
            .get_tile(next_c)
            .ok_or(anyhow!("box move out of bounds"))?
            == Tile::Box
        {
            self.move_box(next_c, d)?;
        }

        // Check if empty space is available.
        if self
            .get_tile(next_c)
            .ok_or(anyhow!("box move out of bounds"))?
            == Tile::Empty
        {
            self.tiles[c.y][c.x] = Tile::Empty;
            self.tiles[next_c.y][next_c.x] = Tile::Box;
        }

        Ok(())
    }

    fn move_robot(&mut self, d: Direction) -> Result<()> {
        let next_c = self.robot.take_step(d);

        // Try to move any blocking box in front.
        if self
            .get_tile(next_c)
            .ok_or(anyhow!("robot move out of bounds"))?
            == Tile::Box
        {
            self.move_box(next_c, d)?;
        }

        // Check if empty space is available.
        if self
            .get_tile(next_c)
            .ok_or(anyhow!("robot move out of bounds"))?
            == Tile::Empty
        {
            self.robot = next_c;
        }

        Ok(())
    }

    fn box_gps_sum(&self) -> usize {
        let mut result = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Box = tile {
                    result += Coord { x, y }.to_gps();
                }
            }
        }
        result
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/15-input.txt")?;
    let (grid_raw, steps_raw) = file.split_once("\n\n").ok_or(anyhow!("invalid input"))?;

    let mut grid = Grid::from_str(grid_raw)?;
    let steps = steps_raw
        .chars()
        .filter(|c| *c != '\n') // ignore newlines
        .map(Direction::from_char)
        .collect::<Result<Vec<_>>>()?;

    for step in steps {
        grid.move_robot(step)?;
    }

    let result = grid.box_gps_sum();
    println!("result: {result}");

    Ok(())
}
