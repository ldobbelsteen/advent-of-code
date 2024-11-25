#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn manhattan_distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() -> Result<()> {
    let y = 2_000_000;
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

    let closest_with_distance: Vec<(Coord, i32, Coord)> = closest
        .into_iter()
        .map(|(sensor, beacon)| {
            let distance = sensor.manhattan_distance(&beacon);
            (sensor, distance, beacon)
        })
        .collect();

    let mut min_x = 0;
    let mut max_x = 0;

    for (sensor, distance, _) in &closest_with_distance {
        let reach_min_x = sensor.x - distance;
        let reach_max_x = sensor.x + distance;

        if reach_min_x < min_x {
            min_x = reach_min_x;
        }
        if reach_max_x > max_x {
            max_x = reach_max_x;
        }
    }

    println!("Min x: {min_x}, Max x: {max_x}");

    let mut blocked_positions = 0;
    for x in min_x..=max_x {
        let coord = Coord { x, y };
        for (sensor, distance, beacon) in &closest_with_distance {
            if coord == *sensor {
                break;
            }

            if coord == *beacon {
                break;
            }

            if sensor.manhattan_distance(&coord) <= *distance {
                blocked_positions += 1;
                break;
            }
        }
    }

    println!("Blocked positions: {blocked_positions}");

    Ok(())
}
