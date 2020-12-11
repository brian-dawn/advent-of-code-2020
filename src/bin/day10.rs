use itertools::Itertools;
use std::{
    collections::{HashMap},
    io::{BufRead, BufReader},
};
use std::{fs::File};

use anyhow::{Context, Result};

fn read_input() -> Result<Vec<i64>> {
    let input = File::open("input/day10.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<i64>()?))
        .collect()
}

fn part1(input: &[i64]) -> i64 {
    let (ones, threes) = input
        .iter()
        .sorted()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .fold((1, 1), |(ones, threes), val| match val {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    ones * threes
}

fn part2(input: &[i64]) -> Option<i64> {
    let mut sorted = input.iter().sorted().copied().collect::<Vec<_>>();
    sorted.push(3 + sorted.last().unwrap_or(&0));

    let mut hits = HashMap::new();

    hits.insert(0, 1);

    for v in &sorted {
        let result = hits.get(&(v - 1)).unwrap_or(&0)
            + hits.get(&(v - 2)).unwrap_or(&0)
            + hits.get(&(v - 3)).unwrap_or(&0);
        println!("{} {:?}", v, result);
        // Carry forward all combos
        hits.insert(*v, result);
    }

    sorted.last().map(|l| hits.get(&l).copied()).flatten()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let answer1 = part1(&input);
    println!("part1: {}", answer1);

    let answer2 = part2(&input).context("Failed to find an answer for part2")?;
    println!("part2: {}", answer2);

    Ok(())
}
