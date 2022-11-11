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
