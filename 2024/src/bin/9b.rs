#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::{collections::HashSet, iter};

#[derive(Debug)]
struct Space {
    start: usize,
    size: usize,
}

#[derive(Debug)]
struct File {
    space: Space,
    id: usize,
}

fn compute_free_spaces(blocks: &[Option<usize>]) -> Vec<Space> {
    let mut free_spaces: Vec<Space> = Vec::new();
    let mut free_start = None;
    for (i, block) in blocks.iter().enumerate() {
        if block.is_none() {
            if free_start.is_none() {
                free_start = Some(i);
            }
        } else if let Some(start) = free_start {
            free_spaces.push(Space {
                start,
                size: i - start,
            });
            free_start = None;
        }
    }
    free_spaces
}

fn first_file_preceding(blocks: &[Option<usize>], mut i: usize) -> Option<File> {
    let mut id = blocks[i];
    while id.is_none() {
        if i == 0 {
            return None;
        }
        i -= 1;
        id = blocks[i];
    }

    let end = i;
    while i > 0 && blocks[i - 1] == id {
        i -= 1;
    }

    Some(File {
        space: Space {
            start: i,
            size: end - i + 1,
        },
        id: id.unwrap(),
    })
}

fn write_id_to_space(blocks: &mut [Option<usize>], space: &Space, id: Option<usize>) {
    for block in blocks.iter_mut().skip(space.start).take(space.size) {
        *block = id;
    }
}

fn compute_checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .fold(0, |checksum, (position, id)| {
            checksum + position * id.unwrap_or(0)
        })
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/9-input.txt")?;
    let input = file
        .strip_suffix('\n')
        .ok_or(anyhow::anyhow!("no newline at end of file"))?;

    let mut blocks = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let id = if i % 2 == 0 { Some(i / 2) } else { None };
        let size = c.to_digit(10).ok_or(anyhow!("not a digit"))?;
        blocks.extend(iter::repeat(id).take(size.try_into()?));
    }

    let mut free_spaces = compute_free_spaces(&blocks);
    let mut handled_files = HashSet::new();
    let mut index = blocks.len() - 1;

    // We iterate over the files in reverse order and try to fit them.
    while let Some(file) = first_file_preceding(&blocks, index) {
        if !handled_files.contains(&file.id) {
            // First, we prune the free spaces not to the left of the file.
            while let Some(last) = free_spaces.last() {
                let end = last.start + last.size - 1;
                if end >= file.space.start {
                    free_spaces.pop();
                } else {
                    break;
                }
            }

            // We find a slot to put the file in.
            for free_space in &mut free_spaces {
                if free_space.size >= file.space.size {
                    let new_file_space = Space {
                        start: free_space.start,
                        size: file.space.size,
                    };

                    free_space.start += file.space.size;
                    free_space.size -= file.space.size;

                    write_id_to_space(&mut blocks, &file.space, None);
                    write_id_to_space(&mut blocks, &new_file_space, Some(file.id));

                    break;
                }
            }
        }

        if file.space.start > 0 {
            index = file.space.start - 1;
            handled_files.insert(file.id);
        } else {
            break;
        }
    }

    let result = compute_checksum(&blocks);
    println!("result: {result}");

    Ok(())
}
