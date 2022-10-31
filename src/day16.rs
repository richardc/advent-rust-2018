use itertools::Itertools;

type Number = u32;

type Instruction = [Number; 4];
type Registers = [Number; 4];

struct Check {
    before: Registers,
    after: Registers,
    instr: Instruction,
}

type Opcode = fn(&Registers, Number, Number, Number) -> Registers;

fn addr(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] + input[b as usize];
    out
}

fn addi(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] + b;
    out
}

fn mulr(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] * input[b as usize];
    out
}

fn muli(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] * b;
    out
}

fn banr(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] & input[b as usize];
    out
}

fn bani(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] & b;
    out
}

fn borr(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] | input[b as usize];
    out
}

fn bori(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize] | b;
    out
}

fn setr(input: &Registers, a: Number, _b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = input[a as usize];
    out
}

fn seti(input: &Registers, a: Number, _b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = a;
    out
}

fn gtir(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = if a > input[b as usize] { 1 } else { 0 };
    out
}

fn gtri(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = if input[a as usize] > b { 1 } else { 0 };
    out
}

fn gtrr(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = if input[a as usize] > input[b as usize] {
        1
    } else {
        0
    };
    out
}

fn eqir(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = if a == input[b as usize] { 1 } else { 0 };
    out
}

fn eqri(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = if input[a as usize] == b { 1 } else { 0 };
    out
}

fn eqrr(input: &Registers, a: Number, b: Number, c: Number) -> Registers {
    let mut out = *input;
    out[c as usize] = if input[a as usize] == input[b as usize] {
        1
    } else {
        0
    };
    out
}

fn opcodes() -> Vec<Opcode> {
    vec![
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ]
}

#[aoc_generator(day16)]
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
            opcodes()
                .iter()
                .filter(|opcode| {
                    check.after
                        == opcode(
                            &check.before,
                            check.instr[1],
                            check.instr[2],
                            check.instr[3],
                        )
                })
                .count()
                >= 3
        })
        .count()
}
