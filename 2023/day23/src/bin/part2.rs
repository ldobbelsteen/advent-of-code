#![warn(clippy::pedantic)]

use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Edge {
    target: usize,
    len: usize,
}

fn coord_to_vertex(x_size: usize, coord: &Coord) -> usize {
    coord.y * x_size + coord.x
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
fn accumulate_graph(
    grid: &Vec<Vec<char>>,
    vertices: &mut HashSet<usize>,
    edges: &mut HashMap<usize, HashSet<Edge>>,

    prev: Option<&Coord>,
    cur: &Coord,
    e_start: usize,
    e_len: usize,

    end: usize,
) {
    let vertex = coord_to_vertex(grid[0].len(), cur);

    let up: Option<Coord> = {
        if cur.y > 0 {
            let coord = Coord {
                x: cur.x,
                y: cur.y - 1,
            };
            if grid[coord.y][coord.x] != '#' && prev != Some(&coord) {
                Some(coord)
            } else {
                None
            }
        } else {
            None
        }
    };

    let down: Option<Coord> = {
        if cur.y < grid.len() - 1 {
            let coord = Coord {
                x: cur.x,
                y: cur.y + 1,
            };
            if grid[coord.y][coord.x] != '#' && prev != Some(&coord) {
                Some(coord)
            } else {
                None
            }
        } else {
            None
        }
    };

    let right: Option<Coord> = {
        if cur.x < grid[0].len() - 1 {
            let coord = Coord {
                x: cur.x + 1,
                y: cur.y,
            };
            if grid[coord.y][coord.x] != '#' && prev != Some(&coord) {
                Some(coord)
            } else {
                None
            }
        } else {
            None
        }
    };

    let left: Option<Coord> = {
        if cur.x > 0 {
            let coord = Coord {
                x: cur.x - 1,
                y: cur.y,
            };
            if grid[coord.y][coord.x] != '#' && prev != Some(&coord) {
                Some(coord)
            } else {
                None
            }
        } else {
            None
        }
    };

    let directions = [&up, &down, &left, &right]
        .iter()
        .filter(|d| d.is_some())
        .count();

    if directions > 1 {
        // Insert edge, since we are at an intersection.
        edges.entry(e_start).or_default().insert(Edge {
            target: vertex,
            len: e_len,
        });
        edges.entry(vertex).or_default().insert(Edge {
            target: e_start,
            len: e_len,
        });

        // If intersection is already (being) visited, do not continue.
        if !vertices.insert(vertex) {
            return;
        }

        // Recurse into all possible directions with new edge starting at current coord.
        if let Some(up) = up {
            accumulate_graph(grid, vertices, edges, Some(cur), &up, vertex, 1, end);
        }
        if let Some(down) = down {
            accumulate_graph(grid, vertices, edges, Some(cur), &down, vertex, 1, end);
        }
        if let Some(right) = right {
            accumulate_graph(grid, vertices, edges, Some(cur), &right, vertex, 1, end);
        }
        if let Some(left) = left {
            accumulate_graph(grid, vertices, edges, Some(cur), &left, vertex, 1, end);
        }
    } else if directions == 1 {
        // Recurse into the only direction, incrementing the edge weight by one.
        if let Some(up) = up {
            accumulate_graph(
                grid,
                vertices,
                edges,
                Some(cur),
                &up,
                e_start,
                e_len + 1,
                end,
            );
        }
        if let Some(down) = down {
            accumulate_graph(
                grid,
                vertices,
                edges,
                Some(cur),
                &down,
                e_start,
                e_len + 1,
                end,
            );
        }
        if let Some(right) = right {
            accumulate_graph(
                grid,
                vertices,
                edges,
                Some(cur),
                &right,
                e_start,
                e_len + 1,
                end,
            );
        }
        if let Some(left) = left {
            accumulate_graph(
                grid,
                vertices,
                edges,
                Some(cur),
                &left,
                e_start,
                e_len + 1,
                end,
            );
        }
    } else if vertex == end {
        edges.entry(e_start).or_default().insert(Edge {
            target: vertex,
            len: e_len,
        });
        edges.entry(vertex).or_default().insert(Edge {
            target: e_start,
            len: e_len,
        });
        vertices.insert(vertex);
    }
}

fn longest_simple_path(
    edges: &HashMap<usize, HashSet<Edge>>,
    start: usize,
    end: usize,
) -> Option<usize> {
    fn rec(
        edges: &HashMap<usize, HashSet<Edge>>,
        visited: &mut HashSet<usize>,
        current: usize,
        end: usize,
    ) -> Option<usize> {
        if current == end {
            return Some(0);
        }
        visited.insert(current);
        let possible_edges: Vec<&Edge> = edges
            .get(&current)
            .unwrap()
            .iter()
            .filter(|e| !visited.contains(&e.target))
            .collect();
        let result = possible_edges
            .iter()
            .filter_map(|e| rec(edges, visited, e.target, end).map(|p| p + e.len))
            .max();
        visited.remove(&current);
        result
    }
    rec(edges, &mut HashSet::new(), start, end)
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let start = Coord {
        x: grid[0].iter().position(|&c| c == '.').unwrap(),
        y: 0,
    };
    let end = Coord {
        x: grid[grid.len() - 1].iter().position(|&c| c == '.').unwrap(),
        y: grid.len() - 1,
    };

    let start_vertex = coord_to_vertex(grid[0].len(), &start);
    let end_vertex = coord_to_vertex(grid[0].len(), &end);

    let mut vertices = HashSet::new();
    let mut edges = HashMap::new();
    accumulate_graph(
        &grid,
        &mut vertices,
        &mut edges,
        None,
        &start,
        start_vertex,
        0,
        end_vertex,
    );

    let result = longest_simple_path(&edges, start_vertex, end_vertex);
    println!("{}", result.unwrap());
}
