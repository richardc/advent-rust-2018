use itertools::Itertools;

fn has_two(s: &str) -> bool {
    s.chars().counts().values().any(|&v| v == 2)
}

fn has_three(s: &str) -> bool {
    s.chars().counts().values().any(|&v| v == 3)
}

#[aoc(day2, part1)]
fn solve(input: &str) -> usize {
    input.lines().filter(|&l| has_three(l)).count() * input.lines().filter(|&l| has_two(l)).count()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(include_str!("day02_example.txt")), 12)
}
