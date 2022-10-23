use std::collections::HashSet;

use itertools::{iproduct, Itertools};
use ndarray::prelude::*;

struct Point {
    x: usize,
    y: usize,
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl Point {
    fn distance_to(&self, x: usize, y: usize) -> usize {
        x.abs_diff(self.x) + y.abs_diff(self.y)
    }
}

#[aoc_generator(day6)]
fn generate(input: &str) -> Vec<Point> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
fn solve(points: &[Point]) -> usize {
    // Generate the zone of distances
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let padding = 2;

    let mut zone: Array2<usize> = Array::zeros((max_x + padding, max_y + padding)).reversed_axes();
    for ((x, y), cell) in zone.indexed_iter_mut() {
        let distances = points.iter().map(|p| p.distance_to(x, y)).collect_vec();
        let min = distances.iter().min().unwrap();
        let with_min = distances.iter().positions(|d| d == min).collect_vec();
        if with_min.len() == 1 {
            *cell = with_min[0] + 1
        }
    }

    // Use 'is on the edge' as a proxy for 'infinite'
    let infinite: HashSet<usize> = [
        zone.row(0).to_vec(),
        zone.column(0).to_vec(),
        zone.row(zone.dim().0 - 1).to_vec(),
        zone.column(zone.dim().1 - 1).to_vec(),
    ]
    .concat()
    .into_iter()
    .collect();

    // All the finite areas
    let areas = zone.iter().filter(|&c| !infinite.contains(c)).counts();
    *areas.values().max().unwrap()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day06_example.txt"))), 17)
}

#[aoc(day6, part2)]
fn solve2(points: &[Point]) -> usize {
    region_sum_below(10000, points)
}

fn region_sum_below(limit: usize, points: &[Point]) -> usize {
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    let mut inside = 0;
    for (x, y) in iproduct!(0..=max_x, 0..=max_y) {
        if points.iter().map(|p| p.distance_to(x, y)).sum::<usize>() < limit {
            inside += 1
        }
    }
    inside
}

#[cfg(test)]
#[test]
fn test_region_sum_below() {
    assert_eq!(
        region_sum_below(32, &generate(include_str!("day06_example.txt"))),
        16
    )
}
