use std::collections::HashSet;

use crate::wasm::Cpu;

#[aoc_generator(day21)]
fn generate(input: &str) -> Cpu {
    input.parse().unwrap()
}

#[aoc(day21, part1)]
fn solve(cpu: &Cpu) -> usize {
    let mut cpu = (*cpu).clone();
    let last_eqrr = cpu.program.len() - 3;
    cpu.run_till_pc(last_eqrr);
    cpu.registers[cpu.program[last_eqrr].a]
}

#[aoc(day21, part2)]
fn solve2(cpu: &Cpu) -> usize {
    let mut cpu = (*cpu).clone();
    let last_eqrr = cpu.program.len() - 3;
    let mut seen: HashSet<usize> = HashSet::new();
    let mut prev = 0;
    loop {
        cpu.run_till_pc(last_eqrr);
        let value = cpu.registers[cpu.program[last_eqrr].a];
        if !seen.insert(value) {
            return prev;
        }
        prev = value;
    }
}
