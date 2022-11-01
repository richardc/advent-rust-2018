use lazy_static::lazy_static;
use ndarray::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Cell {
    #[default]
    Empty,
    Clay,
    Water,
    Reached,
}

#[derive(Debug, Clone)]
struct Well {
    min_row: usize,
    max_row: usize,
    offset: usize,
    cells: Array2<Cell>,
}

impl std::fmt::Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..500 - self.offset {
            write!(f, " ")?;
        }
        writeln!(f, "+")?;
        for r in 0..self.cells.nrows() {
            for c in 0..self.cells.ncols() {
                match self.cells[[r, c]] {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Clay => write!(f, "#")?,
                    Cell::Water => write!(f, "~")?,
                    Cell::Reached => write!(f, "|")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Well {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut clay = vec![];
        for line in s.lines() {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"([xy])=(\d+), [xy]=(\d+)..(\d+)").unwrap();
            }
            let caps = RE.captures(line).unwrap();
            let fixed_axis = caps.get(1).unwrap().as_str();
            let fixed_value = caps.get(2).unwrap().as_str().parse()?;
            let range_from = caps.get(3).unwrap().as_str().parse()?;
            let range_to = caps.get(4).unwrap().as_str().parse()?;

            let grains = (range_from..=range_to).into_iter().map(|ranged_value| {
                if fixed_axis == "y" {
                    (fixed_value, ranged_value)
                } else {
                    (ranged_value, fixed_value)
                }
            });
            clay.extend(grains);
        }

        let offset = clay.iter().map(|g| g.1).min().unwrap() - 1;
        let min_row = clay.iter().map(|g| g.0).min().unwrap();
        let max_row = clay.iter().map(|g| g.0).max().unwrap();
        let rows = max_row + 2;
        let cols = clay.iter().map(|g| g.1).max().unwrap() - offset + 2;
        let mut cells = Array::default((rows, cols));

        clay.iter()
            .for_each(|g| cells[[g.0, g.1 - offset]] = Cell::Clay);

        Ok(Well {
            min_row,
            max_row,
            offset,
            cells,
        })
    }
}

impl Well {
    fn drip(&mut self) {
        self.down((0, 500 - self.offset));
    }

    fn down(&mut self, mut drop: (usize, usize)) {
        use Cell::*;
        loop {
            match self.cells.get(drop) {
                Some(Empty) => *self.cells.get_mut(drop).unwrap() = Reached,
                Some(Water) | Some(Clay) => return self.flood((drop.0 - 1, drop.1)), // Back up and fill?,
                Some(Reached) => {}
                None => return,
            }
            drop.0 += 1;
        }
    }

    fn flood(&mut self, drop: (usize, usize)) {
        use Cell::*;
        // check left
        let mut left_col = drop.1;
        let mut left_drop = false;
        loop {
            match self.cells.get((drop.0 + 1, left_col)) {
                Some(Water) | Some(Clay) => {}
                _ => {
                    left_drop = true;
                    break;
                }
            }
            if let Some(Clay) = self.cells.get((drop.0, left_col)) {
                break;
            }
            left_col -= 1;
        }

        //check right
        let mut right_col = drop.1;
        let mut right_drop = false;
        while right_col < self.cells.dim().1 {
            match self.cells.get((drop.0 + 1, right_col)) {
                Some(Water) | Some(Clay) => {}
                _ => {
                    right_drop = true;
                    break;
                }
            }
            if let Some(Clay) = self.cells.get((drop.0, right_col)) {
                break;
            }
            right_col += 1;
        }

        let mut fill = self.cells.slice_mut(s![drop.0, left_col + 1..right_col]);
        if !right_drop && !left_drop {
            // If we're between walls, record and walk up
            fill.fill(Water);
            return self.flood((drop.0 - 1, drop.1));
        }

        // Otherwise, this is just where water gets to
        fill.fill(Reached);

        if left_drop {
            self.down((drop.0, left_col))
        }

        if right_drop {
            self.down((drop.0, right_col))
        }
    }

    fn reached(&self) -> usize {
        self.cells
            .slice(s![self.min_row..=self.max_row, ..])
            .iter()
            .filter(|&c| matches!(c, Cell::Reached | Cell::Water))
            .count()
    }
}

#[aoc_generator(day17)]
fn generate(input: &str) -> Well {
    input.parse().unwrap()
}

#[aoc(day17, part1)]
fn solve(well: &Well) -> usize {
    let mut well = (*well).clone();
    well.drip();
    println!("{}", well);
    well.reached()
}

#[cfg(test)]
mod solve {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve(&generate(include_str!("day17_example.txt"))), 57);
    }
}
