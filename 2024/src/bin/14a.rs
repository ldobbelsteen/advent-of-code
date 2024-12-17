#![warn(clippy::pedantic)]

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn tick(&mut self, width: isize, height: isize) {
        self.x += self.vx;
        self.y += self.vy;
        self.x = self.x.rem_euclid(width);
        self.y = self.y.rem_euclid(height);
    }
}

#[derive(Debug)]
struct Area {
    width: isize,
    height: isize,
    robots: Vec<Robot>,
}

impl Area {
    fn tick(&mut self) {
        for robot in &mut self.robots {
            robot.tick(self.width, self.height);
        }
    }

    fn safety_factor(&self) -> usize {
        let mut quadrant_count = [0; 4];

        let ignore_center_x = self.width % 2 == 1;
        let ignore_center_y = self.height % 2 == 1;

        for robot in &self.robots {
            let quadrant = match (robot.x < self.width / 2, robot.y < self.height / 2) {
                (true, true) => 0,
                (false, true) => 1,
                (false, false) => 2,
                (true, false) => 3,
            };

            if ignore_center_x && robot.x == self.width / 2 {
                continue;
            }

            if ignore_center_y && robot.y == self.height / 2 {
                continue;
            }

            quadrant_count[quadrant] += 1;
        }

        quadrant_count.iter().product()
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/14-input.txt")?;

    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)")?;

    let robots = re
        .captures_iter(&file)
        .map(|cap| {
            Ok(Robot {
                x: cap[1].parse()?,
                y: cap[2].parse()?,
                vx: cap[3].parse()?,
                vy: cap[4].parse()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut area = Area {
        width: 101,
        height: 103,
        robots,
    };

    for _ in 0..100 {
        area.tick();
    }

    let result = area.safety_factor();
    println!("result: {result}");

    Ok(())
}
