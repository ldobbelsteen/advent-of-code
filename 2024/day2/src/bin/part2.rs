#![warn(clippy::pedantic)]

use anyhow::Result;

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn from_str(s: &str) -> Result<Self> {
        let levels = s
            .split_whitespace()
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<i32>>>()?;
        Ok(Self { levels })
    }

    fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return true;
        }

        if self.levels[0] == self.levels[1] {
            return false;
        }

        let is_increasing = self.levels[0] < self.levels[1];
        for i in 1..self.levels.len() {
            if is_increasing {
                let increase = self.levels[i] - self.levels[i - 1];
                if !(1..=3).contains(&increase) {
                    return false;
                }
            } else {
                let decrease = self.levels[i - 1] - self.levels[i];
                if !(1..=3).contains(&decrease) {
                    return false;
                }
            }
        }

        true
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        for i in 0..self.levels.len() {
            let with_i_dampened = Report {
                levels: [&self.levels[0..i], &self.levels[i + 1..]].concat(),
            };
            if with_i_dampened.is_safe() {
                return true;
            }
        }

        false
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input.txt")?;

    let reports = file
        .lines()
        .map(Report::from_str)
        .collect::<Result<Vec<_>>>()?;

    let result = reports
        .iter()
        .filter(|r| r.is_safe_with_problem_dampener())
        .count();

    println!("result: {result}");

    Ok(())
}
