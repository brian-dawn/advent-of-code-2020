#![feature(iterator_fold_self)]
use std::collections::HashSet;
use std::{fs::File, io::Read};

use anyhow::Result;



fn read_input() -> Result<Vec<Vec<String>>> {
    let mut input = File::open("input/day6.txt")?;

    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    let paragraphs = buffer.split("\n\n");

    return Ok(paragraphs
        .map(|s| {
            s.split("\n")
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect());
}

fn calculate<T>(groups: &Vec<Vec<String>>, combine_fn: T) -> usize
where
    T: Fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
{
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold_first(|acc, val| combine_fn(&acc, &val))
                .unwrap()
                .len()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let part1 = calculate(&input, |a, b| a.union(b).copied().collect());
    println!("part1: {}", part1);

    let part2 = calculate(&input, |a, b| a.intersection(b).copied().collect());
    println!("part2: {}", part2);

    Ok(())
}
