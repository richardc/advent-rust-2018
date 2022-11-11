use crate::wasm::Cpu;

#[aoc_generator(day19)]
fn generate(input: &str) -> Cpu {
    input.parse().unwrap()
}

#[aoc(day19, part1)]
fn solve(cpu: &Cpu) -> usize {
    let mut cpu = (*cpu).clone();
    cpu.run();
    cpu.registers[0]
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day19_example.txt"))), 6)
}

#[aoc(day19, part2)]
fn solve2(cpu: &Cpu) -> usize {
    let mut cpu = (*cpu).clone();
    cpu.registers[0] = 1;
    while cpu.registers[cpu.pc_reg] != 1 {
        cpu.apply();
    }

    let seed = *cpu.registers.iter().max().unwrap();
    let mut total = 0;
    for i in 1..=seed {
        if seed % i == 0 {
            total += i;
        }
    }
    total
}
