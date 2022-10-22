use lazy_static::lazy_static;
use ndarray::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl std::str::FromStr for Claim {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Claim {
            x: caps.get(2).unwrap().as_str().parse()?,
            y: caps.get(3).unwrap().as_str().parse()?,
            w: caps.get(4).unwrap().as_str().parse()?,
            h: caps.get(5).unwrap().as_str().parse()?,
        })
    }
}

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Claim> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day3, part1)]
fn solve(claims: &[Claim]) -> usize {
    let mut fabric: Array2<u32> = Array2::zeros((1000, 1000));
    for claim in claims {
        let mut region =
            fabric.slice_mut(s![claim.x..claim.x + claim.w, claim.y..claim.y + claim.h]);
        region += 1;
    }
    fabric.iter().filter(|&&v| v > 1).count()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day03_example.txt"))), 4)
}
