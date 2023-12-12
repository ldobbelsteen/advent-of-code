#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Grab {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl FromStr for Grab {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let red_re = Regex::new(r"([0-9][0-9]*) red")?;
        let green_re = Regex::new(r"([0-9][0-9]*) green")?;
        let blue_re = Regex::new(r"([0-9][0-9]*) blue")?;

        let red = if let Some(m) = red_re.captures(line).and_then(|cs| cs.get(1)) {
            Some(m.as_str().parse::<u32>()?)
        } else {
            None
        };

        let green = if let Some(m) = green_re.captures(line).and_then(|cs| cs.get(1)) {
            Some(m.as_str().parse::<u32>()?)
        } else {
            None
        };

        let blue = if let Some(m) = blue_re.captures(line).and_then(|cs| cs.get(1)) {
            Some(m.as_str().parse::<u32>()?)
        } else {
            None
        };

        Ok(Grab { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    grabs: Vec<Grab>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self> {
        let re = Regex::new(r"Game ([0-9][0-9]*): (.+)$")?;
        let caps = re
            .captures(line)
            .ok_or(anyhow!("unexpected game format: {}", line))?;
        let id: u32 = caps
            .get(1)
            .ok_or(anyhow!("cannot find game id: {}", line))?
            .as_str()
            .parse()?;
        let grabs_str = caps
            .get(2)
            .ok_or(anyhow!("cannot find grabs: {}", line))?
            .as_str();
        let grabs = grabs_str
            .split("; ")
            .map(Grab::from_str)
            .collect::<Result<Vec<Grab>>>()?;
        Ok(Game { id, grabs })
    }
}

impl Game {
    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        for grab in &self.grabs {
            if let Some(r) = grab.red {
                if r > red {
                    return false;
                }
            }
            if let Some(g) = grab.green {
                if g > green {
                    return false;
                }
            }
            if let Some(b) = grab.blue {
                if b > blue {
                    return false;
                }
            }
        }
        true
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<Game>>>()?;

    let mut result = 0;
    for game in games {
        if game.is_possible(12, 13, 14) {
            result += game.id;
        }
    }

    println!("Result: {result}");
    Ok(())
}
