use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Number = usize;

type Instruction = [Number; 4];
type Registers = [Number; 4];

#[derive(Debug)]
struct Check {
    before: Registers,
    after: Registers,
    instr: Instruction,
}

#[derive(Clone, Copy, EnumIter)]
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
        Gtir => {
            if a > r[b] {
                1
            } else {
                0
            }
        }
        Gtri => {
            if r[a] > b {
                1
            } else {
                0
            }
        }
        Gtrr => {
            if r[a] > r[b] {
                1
            } else {
                0
            }
        }
        Eqir => {
            if a == r[b] {
                1
            } else {
                0
            }
        }
        Eqri => {
            if r[a] == b {
                1
            } else {
                0
            }
        }
        Eqrr => {
            if r[a] == r[b] {
                1
            } else {
                0
            }
        }
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
