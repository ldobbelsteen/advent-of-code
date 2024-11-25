#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn manhattan_distance(&self, other: &Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Diamond {
    center: Coord,
    radius: i64,
}

impl Diamond {
    fn perimeter(&self) -> impl Iterator<Item = Coord> {
        let top = Coord {
            x: self.center.x,
            y: self.center.y + self.radius + 1,
        };
        let bottom = Coord {
            x: self.center.x,
            y: self.center.y - self.radius - 1,
        };
        let left = Coord {
            x: self.center.x - self.radius - 1,
            y: self.center.y,
        };
        let right = Coord {
            x: self.center.x + self.radius + 1,
            y: self.center.y,
        };

        let top_right_perim = (0..=self.radius).map(move |s| Coord {
            x: top.x + s,
            y: top.y - s,
        });

        let top_left_perim = (0..=self.radius).map(move |s| Coord {
            x: left.x + s,
            y: left.y + s,
        });

        let bottom_left_perim = (0..=self.radius).map(move |s| Coord {
            x: bottom.x - s,
            y: bottom.y + s,
        });

        let bottom_right_perim = (0..=self.radius).map(move |s| Coord {
            x: right.x - s,
            y: right.y - s,
        });

        top_right_perim
            .chain(top_left_perim)
            .chain(bottom_left_perim)
            .chain(bottom_right_perim)
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.center.manhattan_distance(coord) <= self.radius
    }
}

fn main() -> Result<()> {
    let min_x = 0;
    let min_y = 0;
    let max_x = 4_000_000;
    let max_y = 4_000_000;
    let file = std::fs::read_to_string("input.txt")?;

    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let closest = file
        .lines()
        .map(|line| {
            let caps = re
                .captures(line)
                .ok_or_else(|| anyhow!("invalid line: {}", line))?;

            let sensor = Coord {
                x: caps[1].parse()?,
                y: caps[2].parse()?,
            };
            let beacon = Coord {
                x: caps[3].parse()?,
                y: caps[4].parse()?,
            };

            Ok((sensor, beacon))
        })
        .collect::<Result<Vec<(Coord, Coord)>>>()?;

    let known_beacons: Vec<Coord> = closest.iter().cloned().map(|(_, beacon)| beacon).collect();

    let diamonds: Vec<Diamond> = closest
        .into_iter()
        .map(|(sensor, beacon)| {
            let radius = sensor.manhattan_distance(&beacon);
            Diamond {
                center: sensor,
                radius,
            }
        })
        .collect();

    for diamond in &diamonds {
        for coord in diamond.perimeter() {
            if coord.x < min_x || coord.x > max_x || coord.y < min_y || coord.y > max_y {
                continue;
            }

            if known_beacons.contains(&coord) {
                continue;
            }

            if diamonds.iter().any(|d| d.contains(&coord)) {
                continue;
            }

            println!(
                "Possible location '{:?}' with tuning frequency {}",
                coord,
                coord.x * 4_000_000 + coord.y
            );
        }
    }

    Ok(())
}
