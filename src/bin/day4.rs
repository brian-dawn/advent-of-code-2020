#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

use anyhow::{bail, Context, Result};

use regex::Regex;

fn read_input() -> Result<Vec<String>> {
    let mut input = File::open("input/day4.txt")?;
    //let buffered = BufReader::new(input);

    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    let paragraphs = buffer.split("\n\n");

    return Ok(paragraphs.map(|s| s.to_owned()).collect());
}

fn as_maps<'a>(passports: &'a [String]) -> Vec<HashMap<&'a str, &'a str>> {
    passports
        .iter()
        .map(|p| {
            p.split_whitespace()
                .map(|e| {
                    let mut parts = e.split(":");
                    Some((parts.next()?, parts.next()?))
                })
                .filter_map(|x| x)
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

fn part1(passports: &[String]) -> usize {
    let keys: Vec<HashSet<_>> = as_maps(passports)
        .iter()
        .map(|kv| kv.keys().copied().collect())
        .collect();

    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect::<HashSet<_>>();

    keys.into_iter()
        .filter(|p| p.is_superset(&required))
        .count()
}

fn validate(passport: &HashMap<&str, &str>) -> Result<()> {
    let byr: i64 = passport.get("byr").context("no byr")?.parse()?;
    if byr < 1920 || byr > 2002 {
        bail!("invalid byr {}", byr)
    }

    let iyr: i64 = passport.get("iyr").context("no iyr")?.parse()?;
    if iyr < 2010 || iyr > 2020 {
        bail!("invalid iyr {}", iyr)
    }

    let eyr: i64 = passport.get("eyr").context("no eyr")?.parse()?;
    if eyr < 2020 || eyr > 2030 {
        bail!("invalid eyr {}", eyr)
    }

    let hgt = passport.get("hgt").context("no hgt")?;
    let hgt_no_cm = hgt.replace("cm", "");
    let hgt_no_in = hgt.replace("in", "");
    if hgt_no_cm.len() < hgt.len() {
        let hgt: i64 = hgt_no_cm.parse()?;
        if hgt < 150 || hgt > 193 {
            bail!("invalid height in cm {}", hgt)
        }
    } else if hgt_no_in.len() < hgt.len() {
        let hgt: i64 = hgt_no_in.parse()?;
        if hgt < 59 || hgt > 76 {
            bail!("invalid height in in, {}", hgt)
        }
    } else {
        bail!("missing units from hgt {}", hgt)
    }

    let hcl = passport.get("hcl").context("no hcl")?;
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
    }
    if !HCL_REGEX.is_match(hcl) {
        bail!("invalid hcl {}", hcl)
    }

    let ecl = passport.get("ecl").context("no ecl")?;
    let valid_ecls = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect::<HashSet<_>>();
    if !valid_ecls.contains(ecl) {
        bail!("invalid ecl {}", ecl)
    }

    let pid = passport.get("pid").context("no pid")?;
    lazy_static! {
        static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    if !PID_REGEX.is_match(pid) {
        bail!("invalid pid {}", pid)
    }

    Ok(())
}
fn part2(passports: &[String]) -> usize {
    let parsed = as_maps(passports);

    parsed
        .into_iter()
        .filter(|p| {
            let result = validate(p);

            result.is_ok()
        })
        .count()
}

fn main() -> Result<()> {
    let input = read_input()?;

    let part1 = part1(&input);
    println!("part1: {}", part1);

    let part2 = part2(&input);
    println!("part2: {}", part2);

    Ok(())
}
