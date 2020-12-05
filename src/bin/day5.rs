use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use itertools::Itertools;

fn read_input() -> Result<Vec<String>> {
    let input = File::open("input/day5.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|l| Ok(l?)).collect()
}

fn seat_id(pass: &str) -> Option<u16> {
    let bin_string = pass
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");

    u16::from_str_radix(&bin_string, 2).ok()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let part1 = input
        .iter()
        .filter_map(|pass| seat_id(pass))
        .max()
        .context("failed to find the biggest id")?;

    println!("part1: {}", part1);

    let part2 = input
        .iter()
        .filter_map(|pass| seat_id(pass))
        .sorted()
        .tuple_windows::<(_, _)>()
        .find(|(a, b)| a + 1 != *b)
        .context("failed to find our seat!")?
        .0
        + 1;

    println!("part2: {}", part2);

    Ok(())
}

#[test]
fn test_seat_ids() {
    assert_eq!(seat_id("BFFFBBFRRR"), Some(567));
    assert_eq!(seat_id("FFFBBBFRRR"), Some(119));
    assert_eq!(seat_id("BBFFBBFRLL"), Some(820));
}
