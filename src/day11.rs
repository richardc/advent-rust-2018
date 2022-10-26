use std::collections::HashMap;

use itertools::iproduct;
use itertools::Itertools;
use ndarray::prelude::*;

fn largest_cell_location(serial: usize) -> (usize, usize) {
    let grid = make_grid(serial);
    let sizes: HashMap<(usize, usize), i32> = iproduct!(0..300 - 3, 0..300 - 3)
        .map(|(x, y)| ((x + 1, y + 1), grid.slice(s![x..x + 3, y..y + 3]).sum()))
        .collect();

    *sizes
        .iter()
        .sorted_by_key(|&(_, v)| v)
        .map(|(k, _)| k)
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
    let (x, y) = largest_cell_location(s.parse().unwrap());
    format!("{},{}", x, y)
}
