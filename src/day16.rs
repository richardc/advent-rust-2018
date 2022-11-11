use std::collections::HashMap;

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

type Number = usize;

type Instruction = [Number; 4];
type Registers = [Number; 4];

#[derive(Debug)]
struct Check {
    before: Registers,
    after: Registers,
    instr: Instruction,
}

#[derive(Debug, Display, Clone, Copy, EnumIter, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn apply(opcode: Opcode, instr: &Instruction, r: &Registers) -> Registers {
    let mut copy = *r;
    let (a, b, c) = (instr[1], instr[2], instr[3]);

    use Opcode::*;
    copy[c] = match opcode {
        Addr => r[a] + r[b],
        Addi => r[a] + b,
        Mulr => r[a] * r[b],
        Muli => r[a] * b,
        Banr => r[a] & r[b],
        Bani => r[a] & b,
        Borr => r[a] | r[b],
        Bori => r[a] | b,
        Setr => r[a],
        Seti => a,
        Gtir => usize::from(a > r[b]),
        Gtri => usize::from(r[a] > b),
        Gtrr => usize::from(r[a] > r[b]),
        Eqir => usize::from(a == r[b]),
        Eqri => usize::from(r[a] == b),
        Eqrr => usize::from(r[a] == r[b]),
    };
    copy
}

#[aoc_generator(day16, part1)]
fn generate(input: &str) -> Vec<Check> {
    let mut checks = vec![];
    for rec in input.lines().collect_vec().chunks(4) {
        if !rec[0].starts_with("Before:") {
            break;
        }
        let (_, before) = rec[0].split_once('[').unwrap();
        let before = before[..before.len() - 1]
            .split(", ")
            .map(|v| v.parse::<Number>().unwrap());
        let instruction = rec[1]
            .split_ascii_whitespace()
            .map(|v| v.parse::<Number>().unwrap());
        let (_, after) = rec[2].split_once('[').unwrap();
        let after = after[..after.len() - 1]
            .split(", ")
            .map(|v| v.parse::<Number>().unwrap());
        checks.push(Check {
            before: before.collect_vec().try_into().unwrap(),
            after: after.collect_vec().try_into().unwrap(),
            instr: instruction.collect_vec().try_into().unwrap(),
        })
    }
    checks
}

#[aoc(day16, part1)]
fn solve(checks: &[Check]) -> usize {
    checks
        .iter()
        .filter(|&check| {
            Opcode::iter()
                .filter(|&opcode| check.after == apply(opcode, &check.instr, &check.before))
                .count()
                >= 3
        })
        .count()
}

#[derive(Debug)]
struct System {
    checks: Vec<Check>,
    program: Vec<Instruction>,
}

#[aoc_generator(day16, part2)]
fn generate2(input: &str) -> System {
    let (_, rest) = input.split_once("\n\n\n\n").unwrap();

    System {
        checks: generate(input),
        program: rest
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|v| v.parse::<Number>().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec(),
    }
}

#[aoc(day16, part2)]
fn solve2(system: &System) -> Number {
    let mut known: HashMap<usize, Opcode> = HashMap::new();
    while known.len() != 16 {
        for check in &system.checks {
            let matching = Opcode::iter()
                .filter(|op| !known.values().contains(op))
                .filter(|op| check.after == apply(*op, &check.instr, &check.before))
                .collect_vec();
            if matching.len() == 1 {
                known.insert(check.instr[0], matching[0]);
            }
        }
    }

    system.program.iter().fold([0; 4], |acc, instr| {
        let opcode = known.get(&instr[0]).unwrap();
        apply(*opcode, instr, &acc)
    })[0]
}
