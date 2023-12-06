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
    category_maps: HashMap<String, AlmanacMap>,
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
            .map(|s| s.parse())
            .tuples()
            .map(|(start, len)| Ok((start?, len?)))
            .map(|r| r.and_then(|(start, len)| Ok(start..start + len)))
            .collect::<Result<Vec<Range<i64>>>>()?;

        let category_maps = caps
            .get(2)
            .unwrap()
            .as_str()
            .split("\n\n")
            .map(AlmanacMap::from_str)
            .map(|m| m.and_then(|m| Ok((m.source_category.clone(), m))))
            .collect::<Result<HashMap<String, AlmanacMap>>>()?;

        Ok(Self {
            seed_ranges,
            category_maps,
        })
    }
}

impl Almanac {
    /// Get the lowest location a range in a category can map to.
    pub fn lowest_location(&self, current_category: &str, mut range: Range<i64>) -> Result<i64> {
        if current_category == "location" {
            return Ok(range.start);
        }

        let map = self
            .category_maps
            .get(current_category)
            .ok_or(anyhow!("category does not exist: {}", current_category))?;

        // Break down the range into subranges which overlap with ranges in the mappings.
        let mut best_location = None;
        while range.start < range.end {
            let range = map.next_subrange(&mut range)?;
            let lowest = self.lowest_location(&map.destination_category, range)?;
            if best_location.map_or(true, |b| lowest < b) {
                best_location = Some(lowest);
            }
        }
        Ok(best_location.unwrap())
    }
}

#[derive(Debug)]
struct AlmanacMap {
    source_category: String,
    destination_category: String,
    ranges: Vec<AlmanacRange>,
}

impl FromStr for AlmanacMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let header = lines.next().ok_or(anyhow!("map is empty: {}", s))?;
        let header_re = Regex::new(r"(.+)-to-(.+) map:")?;
        let header_caps = header_re
            .captures(header)
            .ok_or(anyhow!("invalid map header: {}", header))?;

        let source_category = header_caps.get(1).unwrap().as_str().to_owned();
        let destination_category = header_caps.get(2).unwrap().as_str().to_owned();
        let mut ranges = lines
            .map(AlmanacRange::from_str)
            .collect::<Result<Vec<AlmanacRange>>>()?;

        // Sort the map ranges by the start of the source range,
        // so we can use binary search to find ranges faster.
        ranges.sort_by_key(|r| r.source_range.start);

        Ok(Self {
            source_category,
            destination_category,
            ranges,
        })
    }
}

impl AlmanacMap {
    fn next_subrange(&self, range: &mut Range<i64>) -> Result<Range<i64>> {
        let i = self
            .ranges
            .partition_point(|r| r.source_range.start <= range.start);
        if i == 0 {
            // range start lies before mapping ranges
            if range.end <= self.ranges[0].source_range.start {
                // range has no overlap with mapping ranges, so identical mapping of entire range
                let result = range.start..range.end;
                range.start = range.end;
                Ok(result)
            } else {
                // range start has no overlap, but range tail has overlap with mapping ranges
                let result = range.start..self.ranges[0].source_range.start;
                range.start = self.ranges[0].source_range.start;
                Ok(result)
            }
        } else {
            // range start falls in, between or after mapping ranges
            let first_range = &self.ranges[i - 1];
            if range.end <= first_range.source_range.end {
                // range falls entirely within a mapping range
                let result = range.start + first_range.destination_offset
                    ..range.end + first_range.destination_offset;
                range.start = range.end;
                Ok(result)
            } else if range.start < first_range.source_range.end {
                // range start falls in mapping range, tail falls outside
                let result = range.start + first_range.destination_offset
                    ..first_range.source_range.end + first_range.destination_offset;
                range.start = first_range.source_range.end;
                Ok(result)
            } else {
                // range lies entirely after mapping ranges, so identical mapping of entire range
                let result = range.start..range.end;
                range.start = range.end;
                Ok(result)
            }
        }
    }
}

#[derive(Debug)]
struct AlmanacRange {
    source_range: Range<i64>,
    destination_offset: i64,
}

impl FromStr for AlmanacRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"(\d+) (\d+) (\d+)")?;
        let caps = re.captures(s).ok_or(anyhow!("invalid range: {}", s))?;

        let destination_start = caps.get(1).unwrap().as_str().parse::<i64>()?;
        let source_start = caps.get(2).unwrap().as_str().parse::<i64>()?;
        let range_len = caps.get(3).unwrap().as_str().parse::<i64>()?;

        let source_range = source_start..source_start + range_len;
        let destination_offset = destination_start - source_start;

        Ok(Self {
            source_range,
            destination_offset,
        })
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let almanac = Almanac::from_str(&file)?;

    let best_locations = almanac
        .seed_ranges
        .iter()
        .map(|r| almanac.lowest_location("seed", r.clone()))
        .collect::<Result<Vec<i64>>>()?;

    let result = best_locations.iter().min();
    println!("{:?}", result.unwrap());

    Ok(())
}
