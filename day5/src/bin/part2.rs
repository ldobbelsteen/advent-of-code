use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<Range<i64>>,
    maps: HashMap<String, AlmanacMap>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"seeds: ([\d\s]+)\n\n((?:.|\n)+)")?;
        let caps = re
            .captures(s)
            .ok_or(anyhow!("invalid almanac header: {}", s))?;

        let seed_ranges = caps
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| -> Result<i64> {
                let n = s.parse()?;
                Ok(n)
            })
            .tuples()
            .map(|(start, len)| {
                let start = start?;
                let len = len?;
                Ok(start..start + len)
            })
            .collect::<Result<Vec<Range<i64>>>>()?;

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

        Ok(Self { seed_ranges, maps })
    }
}

impl Almanac {
    pub fn get_best_location(&self, category: &str, mut src_range: Range<i64>) -> Result<i64> {
        if category == "location" {
            return Ok(src_range.start);
        }

        let map = self
            .maps
            .get(category)
            .ok_or(anyhow!("category does not exist: {}", category))?;

        let mut best_location = None;
        while src_range.start < src_range.end {
            let range = map.next_dst_range(&mut src_range)?;
            let range_best = self.get_best_location(&map.dst_category, range)?;
            if best_location.map_or(true, |b| range_best < b) {
                best_location = Some(range_best);
            }
        }
        Ok(best_location.unwrap())
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
        let mut ranges = lines
            .map(AlmanacRange::from_str)
            .collect::<Result<Vec<AlmanacRange>>>()?;
        ranges.sort_by_key(|r| r.src_range.start);

        Ok(Self {
            src_category,
            dst_category,
            ranges,
        })
    }
}

impl AlmanacMap {
    fn next_dst_range(&self, src_range: &mut Range<i64>) -> Result<Range<i64>> {
        let i = self
            .ranges
            .partition_point(|r| r.src_range.start <= src_range.start);
        if i == 0 {
            if src_range.end <= self.ranges[0].src_range.start {
                // range has no overlap with any other range, so identical mapping
                let result = src_range.start..src_range.end;
                src_range.start = src_range.end;
                Ok(result)
            } else {
                // range tail has overlap with (at least) the first range
                let result = src_range.start..self.ranges[0].src_range.start;
                src_range.start = self.ranges[0].src_range.start;
                Ok(result)
            }
        } else {
            // range start falls in, between or after ranges
            let range = &self.ranges[i - 1];
            if src_range.end <= range.src_range.end {
                // range falls entirely within range
                let result = src_range.start + range.dst_offset..src_range.end + range.dst_offset;
                src_range.start = src_range.end;
                Ok(result)
            } else if src_range.start < range.src_range.end {
                // range start falls in range, tail falls outside
                let result =
                    src_range.start + range.dst_offset..range.src_range.end + range.dst_offset;
                src_range.start = range.src_range.end;
                Ok(result)
            } else {
                // range comes entirely after ranges
                let result = src_range.start..src_range.end;
                src_range.start = src_range.end;
                Ok(result)
            }
        }
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

    let best_locations = almanac
        .seed_ranges
        .iter()
        .map(|r| almanac.get_best_location("seed", r.clone()))
        .collect::<Result<Vec<i64>>>()?;

    let result = best_locations.iter().min();
    println!("{:?}", result);

    Ok(())
}
