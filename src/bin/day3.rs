use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use parse_display::{Display, FromStr};

#[derive(Clone, Debug, Copy, PartialEq)]
enum Square {
    open,
    tree,
}

#[derive(Debug)]
struct World {
    width: usize,
    data: Vec<Square>,
}

impl World {
    fn at(&self, x: usize, y: usize) -> Option<Square> {
        let index = y * self.width + x % self.width;
        self.data.get(index).map(|i| *i)
    }
}

fn read_input() -> Result<World> {
    let input = File::open("input/day3.txt")?;
    let buffered = BufReader::new(input);

    let mut data: Vec<Square> = Vec::new();
    let mut width = 0;

    for line in buffered.lines() {
        let line = line?;
        width = line.len();
        for c in line.chars() {
            match c {
                '.' => data.push(Square::open),
                '#' => data.push(Square::tree),
                _ => {
                    bail!("Unrecognized character {}", c);
                }
            }
        }
    }

    return Ok(World { width, data });
}

fn num_trees(world: &World, dx: usize, dy: usize) -> usize {
    let walk = std::iter::successors(Some((0, 0)), |(x, y)| Some((x + dx, y + dy)));

    walk.map(|(x, y)| world.at(x, y))
        .take_while(|square| square.is_some())
        .filter(|square| *square == Some(Square::tree))
        .count()
}

fn main() -> Result<()> {
    let world = read_input()?;

    println!("part1: {}", num_trees(&world, 3, 1));

    let rules = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = rules
        .iter()
        .map(|(dx, dy)| num_trees(&world, *dx, *dy))
        .fold(1, |acc, val| acc * val);

    println!("part2: {}", part2);

    Ok(())
}
