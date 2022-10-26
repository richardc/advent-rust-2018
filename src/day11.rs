use std::collections::HashMap;

use itertools::iproduct;
use itertools::Itertools;
use ndarray::prelude::*;

fn largest_3cell_location(serial: usize) -> (usize, usize) {
    let grid = make_grid(serial);
    let ((x, y), _) = largest_cell_sized(&grid, 3);
    (x, y)
}

fn largest_cell_location(serial: usize) -> (usize, usize, usize) {
    let grid = make_grid(serial);
    let (size, ((x, y), _power)) = (1..300)
        .into_iter()
        .map(|size| (size, largest_cell_sized(&grid, size)))
        .sorted_by_key(|&(_, (_, power))| power)
        .rev()
        .next()
        .unwrap();
    (x, y, size)
}

#[test_case(8 => (3,5,9))]
#[test_case(18 => (90,269,16))]
#[test_case(42 => (232,251,12))]
#[cfg(test)]
fn test_largest_cell_location(serial: usize) -> (usize, usize, usize) {
    largest_cell_location(serial)
}

fn largest_cell_sized(grid: &Array2<i32>, size: usize) -> ((usize, usize), i32) {
    let sizes: HashMap<(usize, usize), i32> = iproduct!(0..300 - size, 0..300 - size)
        .map(|(x, y)| {
            (
                (x + 1, y + 1),
                grid.slice(s![x..x + size, y..y + size]).sum(),
            )
        })
        .collect();

    sizes
        .into_iter()
        .sorted_by_key(|&(_, v)| v)
        .rev()
        .next()
        .unwrap()
}

fn make_grid(serial: usize) -> Array2<i32> {
    let mut grid = Array2::<i32>::zeros((300, 300));
    for ((x, y), c) in grid.indexed_iter_mut() {
        *c = cell_power(x + 1, y + 1, serial);
    }
    grid
}

fn cell_power(x: usize, y: usize, serial: usize) -> i32 {
    let rack = x + 10;
    let power = (rack * y + serial) * rack;
    ((power / 100) % 10) as i32 - 5
}

#[test_case(3, 5, 8 => 4)]
#[test_case(122, 79, 57 => -5)]
#[test_case(217, 196, 39 => 0)]
#[test_case(101, 153, 71 => 4)]
#[cfg(test)]
fn _cell_power(x: usize, y: usize, serial: usize) -> i32 {
    cell_power(x, y, serial)
}

#[aoc(day11, part1)]
fn solve(s: &str) -> String {
    let (x, y) = largest_3cell_location(s.parse().unwrap());
    format!("{},{}", x, y)
}

#[aoc(day11, part2)]
fn solve2(s: &str) -> String {
    let (x, y, size) = largest_cell_location(s.parse().unwrap());
    format!("{},{},{}", x, y, size)
}
