#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
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

    fn from_str(s: &str) -> Result<Outcome> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("invalid outcome string: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    // Determine the shape we should battle this shape with in order to get the given outcome.
    fn outcome_counter(&self, outcome: &Outcome) -> Shape {
        match (self, outcome) {
            (_, Outcome::Draw) => self.clone(),
            (Shape::Rock, Outcome::Win) | (Shape::Scissors, Outcome::Lose) => Shape::Paper,
            (Shape::Rock, Outcome::Lose) | (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Lose) | (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_str(s: &str) -> Result<Shape> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid shape string: {}", s)),
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let rounds: Vec<(Shape, Outcome)> = file
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(opponent, outcome)| {
            (
                Shape::from_str(opponent).unwrap(),
                Outcome::from_str(outcome).unwrap(),
            )
        })
        .collect();

    let mut points = 0;
    for (opponent, outcome) in &rounds {
        let own = opponent.outcome_counter(outcome);
        points += own.score();
        points += outcome.score();
    }

    println!("{points}");
}
