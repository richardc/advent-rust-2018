use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::matrix::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn new(direction: char) -> Self {
        use Direction::*;
        match direction {
            '>' => Right,
            '<' => Left,
            '^' => Up,
            'v' => Down,
            _ => unreachable!(),
        }
    }

    fn turn_left(&self) -> Self {
        use Direction::*;
        match &self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn turn_right(&self) -> Self {
        use Direction::*;
        match &self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        write!(
            f,
            "{}",
            match &self {
                Up => '^',
                Down => 'v',
                Left => '<',
                Right => '>',
            }
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Cart {
    row: usize,
    col: usize,
    facing: Direction,
    turned: usize,
}

impl Cart {
    fn new(row: usize, col: usize, facing: Direction) -> Self {
        Self {
            row,
            col,
            facing,
            turned: 0,
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    fn set_position(&mut self, (row, col): (usize, usize)) {
        self.row = row;
        self.col = col;
    }
}

#[derive(Debug, Clone)]
struct Railway {
    map: Matrix<char>,
    carts: Vec<Cart>,
}

impl std::str::FromStr for Railway {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut map = Matrix::from_rows(input.lines().map(|l| l.chars()))?;
        let carts = map
            .indices()
            .filter_map(|(row, col)| match map.get((row, col)) {
                Some(&c) if "<>v^".contains(c) => {
                    let cart = Some(Cart::new(row, col, Direction::new(c)));
                    *map.get_mut((row, col)).unwrap() = match c {
                        '<' | '>' => '-',
                        'v' | '^' => '|',
                        _ => unreachable!(),
                    };
                    cart
                }
                _ => None,
            })
            .collect();
        Ok(Self { map, carts })
    }
}

impl std::fmt::Display for Railway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let carts: HashMap<(usize, usize), Direction> = self
            .carts
            .iter()
            .map(|c| (c.position(), c.facing))
            .collect();
        for (row, data) in self.map.iter().enumerate() {
            for (col, c) in data.iter().enumerate() {
                if let Some(cart) = carts.get(&(row, col)) {
                    write!(f, "{}", cart)?;
                } else {
                    write!(f, "{}", c)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Railway {
    fn tick(&mut self) -> Option<(usize, usize)> {
        let mut crash = None;
        let mut positions: HashSet<(usize, usize)> =
            self.carts.iter().map(|c| c.position()).collect();
        self.carts
            .iter_mut()
            .sorted_by_key(|c| c.row)
            .for_each(|cart| {
                use Direction::*;
                let next = match &cart.facing {
                    Up => (cart.row - 1, cart.col),
                    Down => (cart.row + 1, cart.col),
                    Left => (cart.row, cart.col - 1),
                    Right => (cart.row, cart.col + 1),
                };

                if let Some(_) = positions.get(&next) {
                    crash = Some(next);
                }
                positions.remove(&cart.position());
                positions.insert(next);
                cart.set_position(next);

                cart.facing = match self.map.get(cart.position()) {
                    Some('/') => match &cart.facing {
                        Up => Right,
                        Down => Left,
                        Left => Down,
                        Right => Up,
                    },
                    Some('\\') => match &cart.facing {
                        Up => Left,
                        Down => Right,
                        Left => Up,
                        Right => Down,
                    },
                    Some('+') => {
                        cart.turned += 1;
                        cart.turned %= 3;
                        match &cart.turned {
                            1 => cart.facing.turn_left(),
                            2 => cart.facing,
                            0 => cart.facing.turn_right(),
                            _ => unreachable!(),
                        }
                    }
                    _ => cart.facing,
                };
            });
        crash
    }
}

#[aoc_generator(day13)]
fn generate(s: &str) -> Railway {
    s.parse().unwrap()
}

#[aoc(day13, part1)]
fn first_crash(railway: &Railway) -> String {
    let mut railway = (*railway).clone();
    loop {
        if let Some(location) = railway.tick() {
            return format!("{},{}", location.1, location.0);
        }
    }
}

#[cfg(test)]
#[test]
fn test_first_crash() {
    assert_eq!(
        first_crash(&generate(include_str!("day13_example.txt"))),
        "7,3"
    )
}
