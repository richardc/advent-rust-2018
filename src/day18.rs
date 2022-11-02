use anyhow::bail;
use itertools::Itertools;
use ndarray::prelude::*;

#[derive(Default, Clone, Copy, PartialEq)]
enum Cell {
    #[default]
    Open,
    Tree,
    Mill,
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Cell::*;
        match value {
            '.' => Ok(Open),
            '|' => Ok(Tree),
            '#' => Ok(Mill),
            _ => bail!("Bad character {:?}", value),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Open => write!(f, "."),
            Tree => write!(f, "|"),
            Mill => write!(f, "#"),
        }
    }
}

#[derive(Clone)]
struct Wood {
    cells: Array2<Cell>,
}

impl std::str::FromStr for Wood {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = input.lines().collect_vec().len();
        let cols = input.lines().next().unwrap().len();
        let data = input
            .chars()
            .filter_map(|c| Cell::try_from(c).ok())
            .collect();

        let cells = Array::from_shape_vec((rows, cols), data)?;

        Ok(Wood { cells })
    }
}

impl std::fmt::Display for Wood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.rows() {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Wood {
    fn neighbours(&self, (row, col): (usize, usize)) -> Vec<Cell> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(|(dr, dc)| {
            let nrow = row as i32 + dr;
            let ncol = col as i32 + dc;
            if nrow < 0 || ncol < 0 {
                None
            } else {
                self.cells.get((nrow as usize, ncol as usize))
            }
        })
        .copied()
        .collect()
    }

    fn tick(&mut self) {
        use Cell::*;
        let mut next = self.cells.clone();
        for ((row, col), cell) in next.indexed_iter_mut() {
            let neighbours = self.neighbours((row, col));
            let trees = neighbours.iter().filter(|c| **c == Tree).count();
            let mills = neighbours.iter().filter(|c| **c == Mill).count();
            *cell = match (*cell, trees, mills) {
                (Open, trees, _) if trees >= 3 => Tree,
                (Tree, _, mills) if mills >= 3 => Mill,
                (Mill, trees, mills) if trees >= 1 && mills >= 1 => Mill,
                (Mill, _, _) => Open,
                (cell, _, _) => cell,
            }
        }
        self.cells = next;
    }

    fn value(&self) -> usize {
        let wood = self.cells.iter().filter(|&c| *c == Cell::Tree).count();
        let mill = self.cells.iter().filter(|&c| *c == Cell::Mill).count();
        wood * mill
    }
}

#[aoc_generator(day18)]
fn generate(input: &str) -> Wood {
    input.parse().unwrap()
}

#[aoc(day18, part1)]
fn solve(wood: &Wood) -> usize {
    let mut wood = (*wood).clone();
    for _ in 0..10 {
        wood.tick();
    }
    println!("{}", wood);
    wood.value()
}

#[cfg(test)]
#[test]
fn solve_example() {
    assert_eq!(solve(&generate(include_str!("day18_example.txt"))), 1147);
}
