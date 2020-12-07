#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
use std::{fs::File};

use anyhow::{Result};
use regex::Regex;



type MapBag = HashMap<String, Vec<(u64, String)>>;

fn read_input() -> Result<MapBag> {
    let input = File::open("input/day7.txt")?;
    let buffered = BufReader::new(input);

    // Feeling lazy today, unwrap and copying here we gooo
    let mapbag = buffered
        .lines()
        .map(|line| line.unwrap())
        .fold(MapBag::new(), |mut acc, val| {
            let name = val.split("bags").next().unwrap().trim();

            lazy_static! {
                static ref RE: Regex = Regex::new(r"(\d+) (.*?) bag").unwrap();
            }
            let bag = RE.captures_iter(&val).map(|cap| {
                let num = cap[1].parse::<u64>().unwrap();
                let color = cap[2].trim().to_owned();
                (num, color)
            });

            acc.insert(name.to_owned(), bag.collect());
            acc
        });
    Ok(mapbag)
}

fn walk(mapbag: &MapBag, visited: &mut HashSet<String>, current_node: &str) -> u64 {
    if visited.contains(current_node) {
        0
    } else {
        visited.insert(current_node.to_string());

        if let Some(to_visit) = mapbag.get(current_node) {
            1 + to_visit
                .iter()
                .map(|(_, visit_name)| walk(&mapbag, visited, visit_name))
                .sum::<u64>()
        } else {
            1
        }
    }
}

fn flip(mapbag: &MapBag) -> MapBag {
    let mut result = MapBag::new();

    // More cloning because I'm lazy. Feels bad but eh.
    for (name, bag) in mapbag {
        for (num, bname) in bag {
            if let Some(existing) = result.get(bname) {
                let mut new_vec = existing.clone();
                new_vec.push((*num, name.clone()));
                result.insert(bname.clone(), new_vec);
            } else {
                result.insert(bname.clone(), vec![(*num, name.clone())]);
            }
        }
    }

    result
}

fn walk2(mapbag: &MapBag, current_node: &str) -> u64 {
    if let Some(to_visit) = mapbag.get(current_node) {
        to_visit
            .iter()
            .map(|(count, node)| {
                //
                count * (walk2(&mapbag, node) + 1)
            })
            .sum()
    } else {
        0
    }
}

fn main() -> Result<()> {
    let input = read_input()?;

    let part1 = walk(&flip(&input), &mut HashSet::new(), "shiny gold") - 1;
    println!("part1: {}", part1);

    let part2 = walk2(&input, "shiny gold");
    println!("part2: {}", part2);
    Ok(())
}
