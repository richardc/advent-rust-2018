use lazy_static::lazy_static;
use ndarray::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: usize,
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
            id: caps.get(1).unwrap().as_str().parse()?,
            x: caps.get(2).unwrap().as_str().parse()?,
            y: caps.get(3).unwrap().as_str().parse()?,
            w: caps.get(4).unwrap().as_str().parse()?,
            h: caps.get(5).unwrap().as_str().parse()?,
        })
    }
}

impl Claim {
    fn region(&self) -> impl ndarray::SliceArg<Ix2> {
        s![self.x..self.x + self.w, self.y..self.y + self.h]
    }
}

#[aoc_generator(day3)]
fn generate(input: &str) -> Vec<Claim> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn mark_fabric(claims: &[Claim]) -> Array2<u32> {
    let mut fabric: Array2<u32> = Array2::zeros((1000, 1000));
    for claim in claims {
        let mut region = fabric.slice_mut(claim.region());
        region += 1;
    }
    fabric
}

#[aoc(day3, part1)]
fn solve(claims: &[Claim]) -> usize {
    mark_fabric(claims).iter().filter(|&&v| v > 1).count()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day03_example.txt"))), 4)
}

#[aoc(day3, part2)]
fn solve2(claims: &[Claim]) -> usize {
    let fabric = mark_fabric(claims);
    for claim in claims {
        let region = fabric.slice(claim.region());
        if region.iter().all(|&v| v == 1) {
            return claim.id;
        }
    }
    unreachable!()
}

#[cfg(test)]
#[test]
fn test_solve2() {
    assert_eq!(solve2(&generate(include_str!("day03_example.txt"))), 3)
}
