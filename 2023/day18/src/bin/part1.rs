#![warn(clippy::pedantic)]

use std::{collections::HashSet, fs};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Bounds {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Bounds {
    fn from_coords(coords: &Vec<Coord>) -> Self {
        let mut bounds = Bounds {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        };

        for coord in coords {
            if coord.x < bounds.x_min {
                bounds.x_min = coord.x;
            }
            if coord.x > bounds.x_max {
                bounds.x_max = coord.x;
            }
            if coord.y < bounds.y_min {
                bounds.y_min = coord.y;
            }
            if coord.y > bounds.y_max {
                bounds.y_max = coord.y;
            }
        }

        bounds
    }

    fn within(&self, coord: &Coord) -> bool {
        coord.x >= self.x_min
            && coord.x <= self.x_max
            && coord.y >= self.y_min
            && coord.y <= self.y_max
    }

    fn on_border(&self, coord: &Coord) -> bool {
        coord.x == self.x_min
            || coord.x == self.x_max
            || coord.y == self.y_min
            || coord.y == self.y_max
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid direction"),
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
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

    fn from(source: &Coord, target: &Coord) -> Direction {
        Self::all()
            .into_iter()
            .find(|d| d.take(target) == *source)
            .unwrap()
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    meters: u32,
}

impl Instruction {
    fn from_line(s: &str) -> Self {
        let mut elements = s.split_whitespace();
        let direction = Direction::from_str(elements.next().unwrap());
        let meters = elements.next().unwrap().parse().unwrap();
        Self { direction, meters }
    }
}

fn enclosed_by_cycle(cycle: &Vec<Coord>) -> HashSet<Coord> {
    let bounds = Bounds::from_coords(cycle);
    let border: HashSet<Coord> = cycle.iter().cloned().collect();
    let mut left: HashSet<Coord> = HashSet::new();
    let mut right: HashSet<Coord> = HashSet::new();

    for i in 0..cycle.len() {
        let predecessor = if i > 0 {
            &cycle[i - 1]
        } else {
            cycle.last().unwrap()
        };
        let current = &cycle[i];
        let successor = if i < cycle.len() - 1 {
            &cycle[i + 1]
        } else {
            cycle.first().unwrap()
        };

        let (left_side, right_side) = perimeter_sides(
            &Direction::from(predecessor, current),
            &Direction::from(successor, current),
        );
        for l in left_side {
            let target = l.take(current);
            if bounds.within(&target) && !border.contains(&target) {
                left.insert(target);
            }
        }
        for r in right_side {
            let target = r.take(current);
            if bounds.within(&target) && !border.contains(&target) {
                right.insert(target);
            }
        }
    }

    flood_fill(&mut right, &border, &bounds);
    flood_fill(&mut left, &border, &bounds);

    if left.iter().any(|t| bounds.on_border(t)) {
        assert!(!right.iter().any(|t| bounds.on_border(t)));
        right
    } else {
        assert!(right.iter().any(|t| bounds.on_border(t)));
        left
    }
}

fn perimeter_sides(
    incoming: &Direction,
    outgoing: &Direction,
) -> (HashSet<Direction>, HashSet<Direction>) {
    let mut left = HashSet::new();
    let mut right = HashSet::new();

    match incoming {
        Direction::Up => {
            left.insert(Direction::Right);
            right.insert(Direction::Left);
        }
        Direction::Down => {
            left.insert(Direction::Left);
            right.insert(Direction::Right);
        }
        Direction::Left => {
            left.insert(Direction::Up);
            right.insert(Direction::Down);
        }
        Direction::Right => {
            left.insert(Direction::Down);
            right.insert(Direction::Up);
        }
    }

    match outgoing {
        Direction::Up => {
            left.insert(Direction::Left);
            right.insert(Direction::Right);
        }
        Direction::Down => {
            left.insert(Direction::Right);
            right.insert(Direction::Left);
        }
        Direction::Left => {
            left.insert(Direction::Down);
            right.insert(Direction::Up);
        }
        Direction::Right => {
            left.insert(Direction::Up);
            right.insert(Direction::Down);
        }
    }

    left.retain(|d| d != incoming && d != outgoing);
    right.retain(|d| d != incoming && d != outgoing);
    (left, right)
}

fn flood_fill(seeds: &mut HashSet<Coord>, border: &HashSet<Coord>, bounds: &Bounds) {
    let mut fresh = seeds.clone();
    loop {
        let mut next_fresh = HashSet::new();
        for coord in fresh {
            for dir in Direction::all() {
                let neighbour = dir.take(&coord);
                if bounds.within(&neighbour)
                    && !seeds.contains(&neighbour)
                    && !border.contains(&neighbour)
                {
                    next_fresh.insert(neighbour);
                }
            }
        }
        fresh = next_fresh.clone();
        if next_fresh.is_empty() {
            break;
        }
        seeds.extend(next_fresh);
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<Instruction> = file.lines().map(Instruction::from_line).collect();

    let mut cycle = Vec::new();
    let mut current = Coord { x: 0, y: 0 };
    for instruction in instructions {
        for _ in 0..instruction.meters {
            current = instruction.direction.take(&current);
            cycle.push(current.clone());
        }
    }

    let enclosed = enclosed_by_cycle(&cycle);
    let result = cycle.len() + enclosed.len();
    println!("{result}");
}
