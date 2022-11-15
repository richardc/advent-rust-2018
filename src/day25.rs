use itertools::Itertools;
use pathfinding::prelude::connected_components;

#[derive(Hash, Clone, PartialEq, Eq)]
struct Point([i32; 4]);

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap(),
        ))
    }
}

impl Point {
    fn distance(&self, other: &Self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(i, v)| v.abs_diff(other.0[i]))
            .sum()
    }
}

#[aoc_generator(day25)]
fn generate(s: &str) -> Vec<Point> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day25, part1)]
fn solve(points: &[Point]) -> usize {
    let connected = connected_components(points, |point| {
        points
            .iter()
            .filter(|p| p.distance(point) <= 3)
            .cloned()
            .collect_vec()
    });
    connected.len()
}

#[cfg(test)]
mod solve {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve(&generate(include_str!("day25_example_1.txt"))), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve(&generate(include_str!("day25_example_2.txt"))), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(solve(&generate(include_str!("day25_example_3.txt"))), 3);
    }

    #[test]
    fn example_4() {
        assert_eq!(solve(&generate(include_str!("day25_example_4.txt"))), 8);
    }
}
