use std::cmp::Ordering;

use itertools::{Either, Itertools};
use pathfinding::prelude::{bfs, bfs_reach, Matrix};

type Health = u8;

#[derive(Clone, Copy, PartialEq)]
enum Force {
    Goblin,
    Elf,
}

#[derive(Clone, Copy)]
struct Unit {
    force: Force,
    health: Health,
}

impl Unit {
    fn new(force: Force, health: Health) -> Self {
        Self { force, health }
    }

    fn take_damage(&self, damage: Health) -> Self {
        let mut copy = *self;
        copy.health = copy.health.saturating_sub(damage);
        copy
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Force::*;
        match self.force {
            Goblin => write!(f, "G"),
            Elf => write!(f, "E"),
        }
    }
}

impl std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Force::*;
        match self.force {
            Goblin => write!(f, "G({})", self.health),
            Elf => write!(f, "E({})", self.health),
        }
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Mob(Unit),
}

impl Cell {
    fn new(c: char) -> Self {
        use Cell::*;
        use Force::*;
        match c {
            '#' => Wall,
            'G' => Mob(Unit::new(Goblin, 200)),
            'E' => Mob(Unit::new(Elf, 200)),
            _ => Empty,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match &self {
            Empty => write!(f, "."),
            Wall => write!(f, "#"),
            Mob(unit) => write!(f, "{}", unit),
        }
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Empty => write!(f, "."),
            Wall => write!(f, "#"),
            Mob(unit) => write!(f, "{:?}", unit),
        }
    }
}

#[derive(Clone)]
struct Game {
    map: Matrix<Cell>,
    round: usize,
}

impl std::str::FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Game {
            round: 0,
            map: Matrix::from_iter(s.lines().map(|l| l.chars().map(Cell::new))),
        })
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        writeln!(f, "Round {}", self.round)?;
        for row in self.map.iter() {
            let mut mobs = vec![];
            for col in row {
                write!(f, "{}", col)?;
                if let Mob(mob) = col {
                    mobs.push(mob);
                }
            }

            if !mobs.is_empty() {
                write!(f, "  ")?;
                for mob in mobs {
                    write!(f, " {:?}", mob)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Game {
    fn winning_team(&self) -> Option<Vec<Health>> {
        use Cell::*;
        use Force::*;
        let (elves, goblins): (Vec<_>, Vec<_>) = self
            .map
            .values()
            .filter(|c| matches!(c, Mob(_)))
            .partition_map(|&cell| match cell {
                Mob(unit) if unit.force == Elf => Either::Left(unit.health),
                Mob(unit) if unit.force == Goblin => Either::Right(unit.health),
                _ => unreachable!(),
            });
        if goblins.is_empty() {
            return Some(elves);
        }
        if elves.is_empty() {
            return Some(goblins);
        }
        None
    }

    fn is_over(&self) -> bool {
        matches!(self.winning_team(), Some(_))
    }

    fn score(&self) -> usize {
        self.winning_team()
            .unwrap_or_default()
            .iter()
            .map(|&h| h as usize)
            .sum::<usize>()
            * self.round
    }

    fn adjacent_enemies(&self, (row, col): (usize, usize), force: Force) -> Vec<(usize, usize)> {
        self.map
                .neighbours((row, col), false)
                .filter(|&cell| matches!(self.map.get(cell), Some(&Cell::Mob(other)) if other.force != force))
                .collect()
    }

    fn successors(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        [
            (row, col + 1),
            (row, col - 1),
            (row - 1, col),
            (row + 1, col),
        ]
        .into_iter()
        .filter(|&p| matches!(self.map.get(p), Some(Cell::Empty)))
        .collect()
    }

    fn reading_order(p1: (usize, usize), p2: (usize, usize)) -> Ordering {
        match Ord::cmp(&p1.0, &p2.0) {
            Ordering::Equal => Ord::cmp(&p1.1, &p2.1),
            ord => ord,
        }
    }

    fn step(&mut self) {
        let mut units = self
            .map
            .indices()
            .filter(|&p| matches!(self.map.get(p), Some(Cell::Mob(_))))
            .collect_vec();

        units.reverse();

        while let Some(mut unit) = units.pop() {
            if self.is_over() {
                return;
            }
            let mob = if let Some(&Cell::Mob(mob)) = self.map.get(unit) {
                mob
            } else {
                panic!()
            };
            let mut enemies = self.adjacent_enemies(unit, mob.force);

            if enemies.is_empty() {
                let targets = bfs_reach(unit, |&p| self.successors(p))
                    .filter_map(|p| {
                        if !self.adjacent_enemies(p, mob.force).is_empty() {
                            Some(p)
                        } else {
                            None
                        }
                    })
                    .collect_vec();

                if targets.is_empty() {
                    continue;
                }

                let route = targets
                    .iter()
                    .filter_map(|&t| bfs(&unit, |&p| self.successors(p), |&p| p == t))
                    .sorted_by(|a, b| match Ord::cmp(&a.len(), &b.len()) {
                        Ordering::Equal => {
                            match Self::reading_order(*a.last().unwrap(), *b.last().unwrap()) {
                                Ordering::Equal => Self::reading_order(a[1], b[1]),
                                ord => ord,
                            }
                        }
                        ord => ord,
                    })
                    .next()
                    .unwrap();

                let step = route[1];
                *self.map.get_mut(step).unwrap() = *self.map.get(unit).unwrap();
                *self.map.get_mut(unit).unwrap() = Cell::Empty;
                unit = step;

                enemies = self.adjacent_enemies(unit, mob.force);
                if enemies.is_empty() {
                    continue;
                }
            }

            // Attack
            let health = enemies
                .iter()
                .map(|&p| match self.map.get(p) {
                    Some(&Cell::Mob(unit)) => (p, unit),
                    _ => unreachable!(),
                })
                .sorted_by_key(|&(_, mob)| mob.health)
                .collect_vec();
            let weakest = health[0].1.health;
            let &(location, target) = health
                .iter()
                .filter(|(_, unit)| unit.health == weakest)
                .sorted_by_key(|&(p, _)| p)
                .next()
                .unwrap();
            let after = target.take_damage(3);
            if after.health == 0 {
                *self.map.get_mut(location).unwrap() = Cell::Empty;
                units.retain(|&p| location != p);
            } else {
                *self.map.get_mut(location).unwrap() = Cell::Mob(after);
            }
        }

        self.round += 1;
    }
}

#[aoc_generator(day15)]
fn generate(input: &str) -> Game {
    input.parse().unwrap()
}

#[aoc(day15, part1)]
fn solve(game: &Game) -> usize {
    println!("{}", game);
    score(game)
}

fn score(game: &Game) -> usize {
    let mut game = (*game).clone();
    while !game.is_over() {
        game.step();
        // println!("{}", game);
    }
    println!("{}", game);
    game.score()
}

#[cfg(test)]
mod score {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(score(&generate(include_str!("day15_example1.txt"))), 27730);
    }

    #[test]
    fn example2() {
        assert_eq!(score(&generate(include_str!("day15_example2.txt"))), 36334);
    }

    #[test]
    fn example3() {
        assert_eq!(score(&generate(include_str!("day15_example3.txt"))), 39514);
    }

    #[test]
    fn example4() {
        assert_eq!(score(&generate(include_str!("day15_example4.txt"))), 27755);
    }

    #[test]
    fn example5() {
        assert_eq!(score(&generate(include_str!("day15_example5.txt"))), 28944);
    }

    #[test]
    fn example6() {
        assert_eq!(score(&generate(include_str!("day15_example6.txt"))), 18740);
    }
}
