#![warn(clippy::pedantic)]

use regex::Regex;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_digit(d: u32) -> Self {
        match d {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("invalid direction digit"),
        }
    }

    fn take(&self, coord: &Coord) -> Coord {
        match self {
            Direction::Down => Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            Direction::Up => Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            Direction::Left => Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            Direction::Right => Coord {
                x: coord.x + 1,
                y: coord.y,
            },
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    meters: u32,
}

impl Instruction {
    fn from_line(s: &str) -> Self {
        let re = Regex::new(r".+ .+ \(#([0-9a-f]{5})([0-9a-f])\)").unwrap();
        let captures = re.captures(s).unwrap();

        let meters = u32::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
        let direction = Direction::from_digit(
            u32::from_str_radix(captures.get(2).unwrap().as_str(), 16).unwrap(),
        );

        Self { direction, meters }
    }
}

fn compute_surface(border: &Vec<Coord>) -> isize {
    let first = border.first().unwrap();
    let last = border.last().unwrap();
    let mut result = 0;

    // Compute surface using Shoelace formula.
    result += first.y * last.x;
    for i in 0..border.len() - 1 {
        result += border[i].x * border[i + 1].y;
    }
    result -= first.x * last.y;
    for i in 0..border.len() - 1 {
        result -= border[i].y * border[i + 1].x;
    }
    result = result.abs();
    result /= 2;

    // Correct for half the border not being included.
    result += isize::try_from(border.len()).unwrap() / 2 + 1;

    result
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut border = Vec::new();
    let mut current = Coord { x: 0, y: 0 };
    for instruction in file.lines().map(Instruction::from_line) {
        for _ in 0..instruction.meters {
            current = instruction.direction.take(&current);
            border.push(current.clone());
        }
    }

    println!("{}", compute_surface(&border));
}
