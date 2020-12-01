use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

fn read_input() -> Result<Vec<i32>> {
    let input = File::open("input/day1.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<i32>()?))
        .collect()
}

fn part1(input: &[i32]) -> Option<i32> {
    return input
        .iter()
        .cartesian_product(input)
        .find(|(a, b)| *a + *b == 2020)
        .map(|(a, b)| a * b);
}

fn part2(input: &[i32]) -> Option<i32> {
    return input
        .iter()
        .cartesian_product(input)
        .cartesian_product(input)
        .find(|((i, j), k)| *i + *j + *k == 2020)
        .map(|((i, j), k)| i * j * k);
}

fn main() -> Result<()> {
    let input = read_input()?;

    let answer1 = part1(&input).context("Failed to find an answer for part1")?;
    println!("part1: {}", answer1);

    let answer2 = part2(&input).context("Failed to find an answer for part2")?;
    println!("part2: {}", answer2);

    Ok(())
}
