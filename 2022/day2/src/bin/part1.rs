#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn battle(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => Outcome::Lose,
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Paper, Shape::Paper)
            | (Shape::Rock, Shape::Rock)
            | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_opponent_str(c: &str) -> Result<Shape> {
        match c {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid opponent shape string: {}", c)),
        }
    }

    fn from_own_str(c: &str) -> Result<Shape> {
        match c {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid own shape string: {}", c)),
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let rounds: Vec<(Shape, Shape)> = file
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent_str, own_str)| {
            let opponent = Shape::from_opponent_str(opponent_str).unwrap();
            let own = Shape::from_own_str(own_str).unwrap();
            (opponent, own)
        })
        .collect();

    let mut points = 0;
    for (opponent, own) in &rounds {
        points += own.score();
        points += own.battle(opponent).score();
    }

    println!("{points}");
}
