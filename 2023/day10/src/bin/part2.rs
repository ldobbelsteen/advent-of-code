#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Bounds {
    x_min: usize,
    x_max: usize, // exclusive
    y_min: usize,
    y_max: usize, // exclusive
}

impl Bounds {
    fn within(&self, coords: &Coord) -> bool {
        coords.x >= self.x_min
            && coords.x < self.x_max
            && coords.y >= self.y_min
            && coords.y < self.y_max
    }

    fn on_border(&self, coords: &Coord) -> bool {
        coords.x == self.x_min
            || coords.x == self.x_min - 1
            || coords.y == self.y_min
            || coords.y == self.y_max - 1
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    E,
    W,
    N,
    S,
    NE,
    NW,
    SE,
    SW,
}

impl Direction {
    fn all() -> [Direction; 8] {
        [
            Self::N,
            Self::W,
            Self::E,
            Self::S,
            Self::NW,
            Self::NE,
            Self::SW,
            Self::SE,
        ]
    }

    /// Move in this direction for a specific coordinate.
    fn take(&self, coord: &Coord) -> Coord {
        match self {
            Self::E => Coord {
                x: coord.x + 1,
                y: coord.y,
            },
            Self::W => Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            Self::N => Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            Self::S => Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            Self::NE => Self::N.take(&Self::E.take(coord)),
            Self::NW => Self::N.take(&Self::W.take(coord)),
            Self::SE => Self::S.take(&Self::E.take(coord)),
            Self::SW => Self::S.take(&Self::W.take(coord)),
        }
    }

    /// Get the direction a source coordinate comes from relative to a target.
    fn from(source: &Coord, target: &Coord) -> Result<Direction> {
        Self::all()
            .into_iter()
            .find(|d| d.take(target) == *source)
            .ok_or(anyhow!(
                "coords are not adjacent: {:?} and {:?}",
                source,
                target
            ))
    }

    /// Get the directions which are adjacent to this direction relative to the same origin.
    fn adjacent(&self) -> HashSet<Direction> {
        let mut result = HashSet::new();
        let center = Coord { x: 1, y: 1 };
        let offset = self.take(&center);

        for center_dir in Self::all() {
            for offset_dir in Self::all() {
                if center_dir.take(&center) == offset_dir.take(&offset) {
                    result.insert(center_dir.clone());
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Connections {
    incoming: Direction,
    outgoing: Direction,
}

#[derive(Debug, Clone)]
struct Sides {
    left: HashSet<Direction>,
    right: HashSet<Direction>,
}

#[derive(Debug, PartialEq)]
enum Tile {
    NorthToSouth,
    WestToEast,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Start,
    Ground,
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '|' => Ok(Self::NorthToSouth),
            '-' => Ok(Self::WestToEast),
            'L' => Ok(Self::NorthToEast),
            'J' => Ok(Self::NorthToWest),
            '7' => Ok(Self::SouthToWest),
            'F' => Ok(Self::SouthToEast),
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            _ => Err(anyhow!("char is not a valid tile: {}", c)),
        }
    }

    /// Get the endpoints of the pipes on this tile.
    fn endpoints(&self) -> HashSet<Direction> {
        match self {
            Self::NorthToSouth => HashSet::from([Direction::N, Direction::S]),
            Self::WestToEast => HashSet::from([Direction::W, Direction::E]),
            Self::NorthToEast => HashSet::from([Direction::N, Direction::E]),
            Self::NorthToWest => HashSet::from([Direction::N, Direction::W]),
            Self::SouthToWest => HashSet::from([Direction::S, Direction::W]),
            Self::SouthToEast => HashSet::from([Direction::S, Direction::E]),
            Self::Start => HashSet::from([Direction::N, Direction::S, Direction::W, Direction::E]),
            Self::Ground => HashSet::from([]),
        }
    }

    /// Get the left and right side of the tile based on which endpoints
    /// the pipes are connected to from the outside.
    fn surrounding_sides(&self, connections: &Connections) -> Result<Sides> {
        let mut result = Sides {
            left: HashSet::new(),
            right: HashSet::new(),
        };

        // Get all eight surrounding tiles and filter out incoming and outgoing connections.
        let mut unmarked = HashSet::from([
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]);
        unmarked.remove(&connections.incoming);
        unmarked.remove(&connections.outgoing);

        // Set the known left and right side of the incoming connection.
        let (left_in, right_in) = match connections.incoming {
            Direction::E => Ok((Direction::SE, Direction::NE)),
            Direction::N => Ok((Direction::NE, Direction::NW)),
            Direction::W => Ok((Direction::NW, Direction::SW)),
            Direction::S => Ok((Direction::SW, Direction::SE)),
            _ => Err(anyhow!(
                "invalid incoming direction for this tile ({:?}): {:?}",
                self,
                connections
            )),
        }?;
        unmarked.remove(&left_in);
        result.left.insert(left_in);
        unmarked.remove(&right_in);
        result.right.insert(right_in);

        // Set the known left and right side of the outgoing connection.
        let (left_out, right_out) = match connections.outgoing {
            Direction::E => Ok((Direction::NE, Direction::SE)),
            Direction::N => Ok((Direction::NW, Direction::NE)),
            Direction::W => Ok((Direction::SW, Direction::NW)),
            Direction::S => Ok((Direction::SE, Direction::SW)),
            _ => Err(anyhow!(
                "invalid outgoing direction for this tile ({:?}): {:?}",
                self,
                connections
            )),
        }?;
        unmarked.remove(&left_out);
        result.left.insert(left_out);
        unmarked.remove(&right_out);
        result.right.insert(right_out);

        // Flood-fill the left and right sides until all are marked.
        while !unmarked.is_empty() {
            result
                .left
                .clone()
                .into_iter()
                .flat_map(|d| d.adjacent())
                .for_each(|adj| {
                    if adj != connections.incoming && adj != connections.outgoing {
                        unmarked.remove(&adj);
                        result.left.insert(adj);
                    }
                });
            result
                .right
                .clone()
                .into_iter()
                .flat_map(|d| d.adjacent())
                .for_each(|adj| {
                    if adj != connections.incoming && adj != connections.outgoing {
                        unmarked.remove(&adj);
                        result.right.insert(adj);
                    }
                });
        }

        Ok(result)
    }
}

#[derive(Debug)]
struct Grid {
    start: Coord,
    tiles: Vec<Vec<Tile>>,
    bounds: Bounds,
}

impl Grid {
    /// Get the adjacent coordinates to a pipe at a coordinate. Only returns coordinates
    /// actually within the bounds and actually at endpoints of the pipe.
    fn adjacent(&self, coord: &Coord) -> HashSet<Coord> {
        self.tiles[coord.y][coord.x]
            .endpoints()
            .into_iter()
            .map(|d| d.take(coord))
            .filter(|c| self.bounds.within(c))
            .collect()
    }

    /// Get the adjacent coordinates which have tiles where the endpoints match
    /// e.g. they can connect and form a cycle.
    fn matching_adjacent(&self, coord: &Coord) -> HashSet<Coord> {
        self.adjacent(coord)
            .into_iter()
            .filter(|c| self.adjacent(c).contains(coord))
            .collect()
    }

    /// Find a loop/cycle of pipes starting from the grid's starting point.
    fn find_cycle(&self) -> Result<Vec<Coord>> {
        fn depth_first_search(
            grid: &Grid,
            previous: &Option<Coord>,
            current: &Coord,
            path: &mut Vec<Coord>,
        ) -> bool {
            path.push(current.clone());
            for next in grid.matching_adjacent(current) {
                if let Some(previous) = &previous {
                    if next == *previous {
                        continue; // circling back of not valid
                    }
                }
                if next == grid.start {
                    return true; // we have come back to the start and are finished
                }
                if depth_first_search(grid, &Some(current.clone()), &next, path) {
                    return true; // if this branch succeeded down the line, we are finished
                }
            }
            path.pop(); // none of the branches succeeded, thus remove the element again
            false
        }

        let mut result = Vec::new();
        if depth_first_search(self, &None, &self.start, &mut result) {
            Ok(result)
        } else {
            Err(anyhow!("no cycle could be found in grid: {:?}", self))
        }
    }

    /// Get the tiles enclosed by a cycle. Marks the bordering tiles on the inside
    /// and outside and flood-fills the desired enclosed space.
    fn enclosed_by_cycle(&self, cycle: &Vec<Coord>) -> Result<HashSet<Coord>> {
        let cycle_coords: HashSet<Coord> = cycle.iter().cloned().collect();
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

            let connections = Connections {
                incoming: Direction::from(predecessor, current)?,
                outgoing: Direction::from(successor, current)?,
            };

            let sides = self.tiles[current.y][current.x].surrounding_sides(&connections)?;
            for l in sides.left {
                let target = l.take(current);
                if self.bounds.within(&target) && !cycle_coords.contains(&target) {
                    left.insert(target);
                }
            }
            for r in sides.right {
                let target = r.take(current);
                if self.bounds.within(&target) && !cycle_coords.contains(&target) {
                    right.insert(target);
                }
            }
        }

        self.flood_fill(&mut right, &cycle_coords);
        self.flood_fill(&mut left, &cycle_coords);

        if left.iter().any(|t| self.bounds.on_border(t)) {
            assert!(!right.iter().any(|t| self.bounds.on_border(t)));
            Ok(right)
        } else {
            assert!(right.iter().any(|t| self.bounds.on_border(t)));
            Ok(left)
        }
    }

    /// Flood-fill a set of seeds with a specified border.
    fn flood_fill(&self, fill: &mut HashSet<Coord>, border: &HashSet<Coord>) {
        let mut fresh = fill.clone();
        loop {
            let mut next_fresh = HashSet::new();
            for coord in fresh {
                for dir in Direction::all() {
                    let neighbour = dir.take(&coord);
                    if self.bounds.within(&neighbour)
                        && !fill.contains(&neighbour)
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
            fill.extend(next_fresh);
        }
    }

    /// Print a cycle along with its enclosed area to standard output.
    fn print_enclosed(&self, cycle: &Vec<Coord>, enclosed: &HashSet<Coord>) {
        let mut result: Vec<Vec<char>> = (self.bounds.y_min..self.bounds.y_max)
            .map(|_| vec!['.'; self.bounds.x_max - self.bounds.x_min])
            .collect();
        for coord in cycle {
            result[coord.y][coord.x] = 'x';
        }
        for coord in enclosed {
            result[coord.y][coord.x] = 'I';
        }
        for row in result {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::from_char)
                    .collect::<Result<Vec<Tile>>>()
            })
            .collect::<Result<Vec<Vec<Tile>>>>()?;
        let start_row = tiles
            .iter()
            .position(|row| row.contains(&Tile::Start))
            .ok_or(anyhow!("no starting point found in grid: {}", s))?;
        let start_col = tiles[start_row]
            .iter()
            .position(|t| *t == Tile::Start)
            .unwrap();
        Ok(Grid {
            start: Coord {
                x: start_col,
                y: start_row,
            },
            bounds: Bounds {
                x_min: 0,
                x_max: tiles[0].len(),
                y_min: 0,
                y_max: tiles.len(),
            },
            tiles,
        })
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let grid = Grid::from_str(&file)?;
    let cycle = grid.find_cycle()?;
    let enclosed = grid.enclosed_by_cycle(&cycle)?;
    grid.print_enclosed(&cycle, &enclosed);
    println!("{}", enclosed.len());
    Ok(())
}
