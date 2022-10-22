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

fn one_different(a: &str, b: &str) -> Option<String> {
    let a = a.as_bytes();
    let b = b.as_bytes();
    for i in 0..a.len() {
        if a[..i] == b[..i] && a[i + 1..] == b[i + 1..] {
            return Some(format!(
                "{}{}",
                std::str::from_utf8(&a[..i]).unwrap(),
                std::str::from_utf8(&a[i + 1..]).unwrap()
            ));
        }
    }
    None
}

#[aoc(day2, part2)]
fn solve2(input: &str) -> String {
    input
        .lines()
        .permutations(2)
        .find_map(|v| one_different(v[0], v[1]))
        .unwrap()
}

#[cfg(test)]
#[test]
fn test_solve2() {
    assert_eq!(solve2(include_str!("day02_example2.txt")), "fgij")
}
