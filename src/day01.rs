use std::collections::HashSet;

#[aoc_generator(day1)]
fn generate(s: &str) -> Vec<i32> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn solve2(input: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    let mut freq = 0;
    seen.insert(freq);
    loop {
        for reading in input {
            freq += reading;
            if !seen.insert(freq) {
                return freq;
            }
        }
    }
}

#[test_case(&[1, -1] => 0)]
#[test_case(&[3, 3, 4, -2, -4] => 10)]
#[test_case(&[-6, 3, 8, 5, -6] => 5)]
#[test_case(&[7, 7, -2, -7, -4] => 14)]
#[cfg(test)]
fn _solve2(input: &[i32]) -> i32 {
    solve2(input)
}
