use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

#[derive(Debug)]
struct PasswordRow {
    start: usize,
    end: usize,
    req: char,
    password: String,
}

impl PasswordRow {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.req).count();

        count <= self.end && count >= self.start
    }

    fn is_valid_part_2(&self) -> bool {
        let present_pos_1 = self.password.chars().nth(self.start - 1) == Some(self.req);
        let present_pos_2 = self.password.chars().nth(self.end - 1) == Some(self.req);
        present_pos_1 ^ present_pos_2
    }

    fn parse(line: &str) -> Result<PasswordRow> {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        let caps = re.captures(&line).context("failed to capture regex")?;
        Ok(PasswordRow {
            start: caps[1].parse::<usize>()?,
            end: caps[2].parse::<usize>()?,
            req: caps[3].chars().next().context("failed to parse")?,
            password: caps[4].to_owned(),
        })
    }
}

fn read_input() -> Result<Vec<PasswordRow>> {
    let input = File::open("input/day2.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| {
            let line = line?;
            PasswordRow::parse(&line)
        })
        .collect()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let answer1 = input.iter().filter(|e| e.is_valid()).count();
    println!("part1: {}", answer1);

    let answer2 = input.iter().filter(|e| e.is_valid_part_2()).count();
    println!("part2: {}", answer2);

    Ok(())
}
