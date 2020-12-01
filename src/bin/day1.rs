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
    for (ix, x) in input.iter().enumerate() {
        for (iy, y) in input.iter().enumerate() {
            if iy == ix {
                continue;
            }
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }

    None
}

fn part2(input: &[i32]) -> Option<i32> {
    for (ix, x) in input.iter().enumerate() {
        for (iy, y) in input.iter().enumerate() {
            if iy == ix || x + y > 2020 {
                continue;
            }
            for (iz, z) in input.iter().enumerate() {
                if iz == ix || iz == iy {
                    continue;
                }
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let input = read_input()?;

    let answer1 = part1(&input).context("Failed to find an answer for part1")?;
    println!("part1: {}", answer1);

    let answer2 = part2(&input).context("Failed to find an answer for part2")?;
    println!("part2: {}", answer2);

    Ok(())
}
