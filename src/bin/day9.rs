use itertools::{Itertools, MinMaxResult};
use std::io::{BufRead, BufReader};
use std::{collections::HashSet, fs::File};

use anyhow::{Context, Result};

fn read_input() -> Result<Vec<u64>> {
    let input = File::open("input/day9.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<u64>()?))
        .collect()
}

fn part1(input: &[u64]) -> Option<u64> {
    let preamble_size = 25;

    // NOTE: Yes I know this is slow and has a lot of recomputation.
    input
        .windows(preamble_size + 1)
        .find(|window| {
            // Find the first window where this holds true
            let cache = window
                .iter()
                .take(preamble_size)
                .combinations(2)
                .map(|ab| ab[0] + ab[1])
                .collect::<HashSet<u64>>();

            if let Some(last) = window.last() {
                !cache.contains(last)
            } else {
                false
            }
        })
        .and_then(|window| window.last())
        .copied()
}

fn part2(input: &[u64]) -> Option<u64> {
    let number_to_find = part1(&input)?;

    (2..input.len())
        .map(|window_size| {
            input
                .windows(window_size)
                .find(|window| window.iter().sum::<u64>() == number_to_find)
                .map(|window| {
                    if let MinMaxResult::MinMax(a, b) = window.iter().minmax() {
                        a + b
                    } else {
                        // Won't ever happen.
                        0
                    }
                })
        })
        .find(|sum| sum.is_some())
        .flatten()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let answer1 = part1(&input).context("Failed to find an answer for part1")?;
    println!("part1: {}", answer1);

    let answer2 = part2(&input).context("Failed to find an answer for part2")?;
    println!("part2: {}", answer2);

    Ok(())
}
