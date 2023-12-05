use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    maps: HashMap<String, AlmanacMap>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"seeds: ([\d\s]+)\n\n((?:.|\n)+)")?;
        let caps = re
            .captures(s)
            .ok_or(anyhow!("invalid almanac header: {}", s))?;

        let seeds = caps
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| -> Result<i64> {
                let n = s.parse()?;
                Ok(n)
            })
            .collect::<Result<Vec<i64>>>()?;

        let maps = caps
            .get(2)
            .unwrap()
            .as_str()
            .split("\n\n")
            .map(|s| -> Result<(String, AlmanacMap)> {
                let map = AlmanacMap::from_str(s)?;
                Ok((map.src_category.clone(), map))
            })
            .collect::<Result<HashMap<String, AlmanacMap>>>()?;

        Ok(Self { seeds, maps })
    }
}

#[derive(Debug)]
struct AlmanacMap {
    src_category: String,
    dst_category: String,
    ranges: Vec<AlmanacRange>,
}

impl FromStr for AlmanacMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let header = lines.next().ok_or(anyhow!("empty map: {}", s))?;
        let header_re = Regex::new(r"(.+)-to-(.+) map:")?;
        let header_caps = header_re
            .captures(header)
            .ok_or(anyhow!("invalid map header: {}", header))?;

        let src_category = header_caps.get(1).unwrap().as_str().to_owned();
        let dst_category = header_caps.get(2).unwrap().as_str().to_owned();
        let ranges = lines
            .map(AlmanacRange::from_str)
            .collect::<Result<Vec<AlmanacRange>>>()?;

        Ok(Self {
            src_category,
            dst_category,
            ranges,
        })
    }
}

impl AlmanacMap {
    fn get_dst(&self, src: i64) -> i64 {
        for range in &self.ranges {
            if range.src_range.contains(&src) {
                return src + range.dst_offset;
            }
        }
        src
    }
}

#[derive(Debug)]
struct AlmanacRange {
    src_range: Range<i64>,
    dst_offset: i64,
}

impl FromStr for AlmanacRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"(\d+) (\d+) (\d+)")?;
        let caps = re.captures(s).ok_or(anyhow!("invalid range: {}", s))?;

        let dst_start = caps.get(1).unwrap().as_str().parse::<i64>()?;
        let src_start = caps.get(2).unwrap().as_str().parse::<i64>()?;
        let len = caps.get(3).unwrap().as_str().parse::<i64>()?;

        let src_range = src_start..src_start + len;
        let dst_offset = dst_start - src_start;

        Ok(Self {
            src_range,
            dst_offset,
        })
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let almanac = Almanac::from_str(&file)?;

    let mut best_location = None;
    for seed in almanac.seeds {
        let mut category = "seed";
        let mut number = seed;

        while category != "location" {
            let map = almanac
                .maps
                .get(category)
                .ok_or(anyhow!("unknown map key: {}", category))?;
            category = &map.dst_category;
            number = map.get_dst(number);
        }

        if best_location.map_or(true, |best| number < best) {
            best_location = Some(number)
        }
    }

    println!("{:?}", best_location);
    Ok(())
}
