#![warn(clippy::pedantic)]
#![feature(int_roundings)]

use std::cmp::{max, min, Ordering};
use std::fs;

#[derive(Clone, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Hailstone {
    start: Coord,
    velocity: Velocity,
}

#[derive(Debug)]
struct LineSegment {
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
struct Bounds {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Colinear,
    Clockwise,
    Counterclockwise,
}

fn on_line_segment(ls: &LineSegment, q: &Coord) -> bool {
    (q.x <= max(ls.start.x, ls.end.x))
        && (q.x >= min(ls.start.x, ls.end.x))
        && (q.y <= max(ls.start.y, ls.end.y))
        && (q.y >= min(ls.start.y, ls.end.y))
}

fn triple_orientation(first: &Coord, second: &Coord, third: &Coord) -> Orientation {
    let v = ((second.y - first.y) * (third.x - second.x))
        - ((second.x - first.x) * (third.y - second.y));
    match v.cmp(&0) {
        Ordering::Greater => Orientation::Clockwise,
        Ordering::Less => Orientation::Counterclockwise,
        Ordering::Equal => Orientation::Colinear,
    }
}

fn line_segments_intersect(first: &LineSegment, second: &LineSegment) -> bool {
    let o1 = triple_orientation(&first.start, &first.end, &second.start);
    let o2 = triple_orientation(&first.start, &first.end, &second.end);
    let o3 = triple_orientation(&second.start, &second.end, &first.start);
    let o4 = triple_orientation(&second.start, &second.end, &first.end);

    if (o1 != o2) && (o3 != o4) {
        return true;
    }

    if o1 == Orientation::Colinear && on_line_segment(first, &second.start) {
        return true;
    }

    if o2 == Orientation::Colinear && on_line_segment(first, &second.end) {
        return true;
    }

    if o3 == Orientation::Colinear && on_line_segment(second, &first.start) {
        return true;
    }

    if o4 == Orientation::Colinear && on_line_segment(second, &first.end) {
        return true;
    }

    false
}

fn in_bounds(c: &Coord, b: &Bounds) -> bool {
    c.x >= b.x_min && c.x <= b.x_max && c.y >= b.y_min && c.y <= b.y_max
}

fn hailstone_trajectory(h: &Hailstone, b: &Bounds) -> Option<LineSegment> {
    // If startpoint not in bounds, move it inside if possible.
    let start = if in_bounds(&h.start, b) {
        h.start.clone()
    } else {
        let bound_x = {
            if h.velocity.x < 0 {
                b.x_max
            } else {
                b.x_min
            }
        };
        let bound_y = {
            if h.velocity.y < 0 {
                b.y_max
            } else {
                b.y_min
            }
        };
        let x_t = (bound_x - h.start.x).div_floor(h.velocity.x);
        if x_t < 0 {
            return None;
        }
        let y_t = (bound_y - h.start.y).div_floor(h.velocity.y);
        if y_t < 0 {
            return None;
        }
        let t = max(x_t, y_t);
        Coord {
            x: h.start.x + t * h.velocity.x,
            y: h.start.y + t * h.velocity.y,
        }
    };

    // Get endpoint that is on the border of the bounds.
    let end = {
        let bound_x = {
            if h.velocity.x < 0 {
                b.x_min
            } else {
                b.x_max
            }
        };
        let bound_y = {
            if h.velocity.y < 0 {
                b.y_min
            } else {
                b.y_max
            }
        };
        let x_t = (bound_x - h.start.x).div_ceil(h.velocity.x);
        if x_t <= 0 {
            return None;
        }
        let y_t = (bound_y - h.start.y).div_ceil(h.velocity.y);
        if y_t <= 0 {
            return None;
        }
        let t = min(x_t, y_t);
        Coord {
            x: h.start.x + t * h.velocity.x,
            y: h.start.y + t * h.velocity.y,
        }
    };

    Some(LineSegment { start, end })
}

fn main() {
    let bounds = Bounds {
        x_min: 200_000_000_000_000,
        y_min: 200_000_000_000_000,
        x_max: 400_000_000_000_000,
        y_max: 400_000_000_000_000,
    };
    let file = fs::read_to_string("input.txt").unwrap();

    let hailstones: Vec<Hailstone> = file
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" @ ").unwrap();
            let left = left.replace(',', " ");
            let right = right.replace(',', " ");
            let mut left_values = left.split_whitespace();
            let mut right_values = right.split_whitespace();
            Hailstone {
                start: Coord {
                    x: left_values.next().unwrap().parse().unwrap(),
                    y: left_values.next().unwrap().parse().unwrap(),
                },
                velocity: Velocity {
                    x: right_values.next().unwrap().parse().unwrap(),
                    y: right_values.next().unwrap().parse().unwrap(),
                },
            }
        })
        .collect();

    let trajectories: Vec<LineSegment> = hailstones
        .iter()
        .filter_map(|h| hailstone_trajectory(h, &bounds))
        .collect();

    let mut result = 0;
    for i in 0..trajectories.len() {
        for j in i + 1..trajectories.len() {
            if line_segments_intersect(&trajectories[i], &trajectories[j]) {
                result += 1;
            }
        }
    }
    println!("{result}");
}
