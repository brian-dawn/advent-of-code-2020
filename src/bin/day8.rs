use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::{convert::TryInto, fs::File};

use anyhow::{Context, Result};
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Clone, Copy)]
enum Instruction {
    #[display("nop {0}")]
    Nop(i64),

    #[display("acc {0}")]
    Acc(i64),

    #[display("jmp {0}")]
    Jmp(i64),
}

impl Instruction {
    fn flip(self) -> Instruction {
        match self {
            Instruction::Nop(a) => Instruction::Jmp(a),
            Instruction::Acc(a) => Instruction::Acc(a),
            Instruction::Jmp(a) => Instruction::Nop(a),
        }
    }
}

fn read_input() -> Result<Vec<Instruction>> {
    let input = File::open("input/day8.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|l| {
            let parsed = l?.replace("+", "").parse();
            Ok(parsed?)
        })
        .collect()
}

enum State {
    InfiniteLoop(i64),
    Terminated(i64),
}

fn part1(program: &[Instruction]) -> State {
    let mut acc = 0;
    let mut pc = 0;

    let mut instructions_ran = HashSet::new();
    loop {
        if instructions_ran.contains(&pc) {
            return State::InfiniteLoop(acc);
        }
        instructions_ran.insert(pc);

        let upc: usize = pc.try_into().unwrap();
        if let Some(current_instruction) = program.get(upc) {
            match current_instruction {
                Instruction::Nop(_) => pc += 1,
                Instruction::Acc(a) => {
                    pc += 1;
                    acc += a
                }
                Instruction::Jmp(offset) => pc += offset,
            }
        } else {
            break;
        }
    }

    State::Terminated(acc)
}

fn part2(program: &[Instruction]) -> Option<i64> {
    let mut altered = program.to_vec();
    for (index, instruction) in program.iter().enumerate() {
        altered[index] = instruction.flip();
        if index > 0 {
            // Revert the old one
            altered[index - 1] = altered[index - 1].flip();
        }
        match part1(&altered) {
            State::InfiniteLoop(_) => {}
            State::Terminated(acc) => {
                return Some(acc);
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let program = read_input()?;
    if let State::InfiniteLoop(part1) = part1(&program) {
        println!("part1: {}", part1);
    }

    let part2 = part2(&program).context("failed to fix the program")?;
    println!("part2: {}", part2);
    Ok(())
}
