use std::{cmp::Ordering, collections::HashMap};

use itertools::{Either, Itertools};
use pathfinding::prelude::{build_path, dijkstra_all, Matrix};

type Health = u8;

#[derive(Clone, Copy, PartialEq)]
enum Force {
    Goblin,
    Elf,
}

#[derive(Clone, Copy, PartialEq)]
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

    fn heal(&mut self) {
        self.health = 200
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, PartialEq)]
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

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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

    fn elves(&self) -> usize {
        self.map
            .values()
            .filter(|c| matches!(c, Cell::Mob(mob) if mob.force == Force::Elf))
            .count()
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

    fn soft_reset(&mut self) {
        self.round = 0;
        for cell in self.map.iter_mut() {
            match cell {
                Cell::Mob(mob) => mob.heal(),
                _ => {}
            }
        }
    }

    fn adjacent_enemies(&self, location: (usize, usize), force: Force) -> Vec<(usize, usize)> {
        self.map
                .neighbours(location, false)
                .filter(|&cell| matches!(self.map.get(cell), Some(&Cell::Mob(other)) if other.force != force))
                .collect()
    }

    fn successors_costed(&self, (row, col): (usize, usize)) -> Vec<((usize, usize), usize)> {
        [
            ((row - 1, col), 1), // Up
            ((row, col - 1), 2), // Left
            ((row, col + 1), 3), // Right
            ((row + 1, col), 4), // Down
        ]
        .into_iter()
        .filter(|&(p, _)| matches!(self.map.get(p), Some(Cell::Empty)))
        .collect()
    }

    fn reading_order(p1: (usize, usize), p2: (usize, usize)) -> Ordering {
        match Ord::cmp(&p1.0, &p2.0) {
            Ordering::Equal => Ord::cmp(&p1.1, &p2.1),
            ord => ord,
        }
    }

    fn unit_at(&self, p: (usize, usize)) -> Unit {
        match self.map.get(p) {
            Some(&Cell::Mob(mob)) => mob,
            _ => unreachable!(),
        }
    }

    fn move_for(&self, location: (usize, usize)) -> Option<(usize, usize)> {
        let mob = self.unit_at(location);
        if !self.adjacent_enemies(location, mob.force).is_empty() {
            // No need to move, we're in striking range
            return None;
        }

        let reachable: HashMap<(usize, usize), ((usize, usize), usize)> =
            dijkstra_all(&location, |&p| self.successors_costed(p));

        let targets = reachable
            .keys()
            .filter(|&p| !self.adjacent_enemies(*p, mob.force).is_empty())
            .collect_vec();

        if targets.is_empty() {
            // Cannot path adjacent to an enemy
            return None;
        }

        let route = targets
            .iter()
            .map(|&t| build_path(t, &reachable))
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

        Some(route[1])
    }
}

#[cfg(test)]
mod game {
    use super::*;
    mod move_for {
        use super::*;

        #[test_case((1,1) => Some((1,2)))]
        #[test_case((1,4) => Some((2,4)))]
        #[test_case((1,7) => Some((1,6)))]
        #[test_case((4,1) => Some((4,2)))]
        fn example(location: (usize, usize)) -> Option<(usize, usize)> {
            let game = generate(include_str!("day15_example_move.txt"));
            game.move_for(location)
        }
    }
}

impl Game {
    fn step(&mut self, elfbuff: u8) {
        let mut units = self
            .map
            .indices()
            .filter(|&p| matches!(self.map.get(p), Some(Cell::Mob(_))))
            .collect_vec();

        units.reverse();

        while let Some(mut location) = units.pop() {
            if self.is_over() {
                return;
            }

            if let Some(step) = self.move_for(location) {
                *self.map.get_mut(step).unwrap() = *self.map.get(location).unwrap();
                *self.map.get_mut(location).unwrap() = Cell::Empty;
                location = step;
            }

            let mob = self.unit_at(location);
            let enemies = self.adjacent_enemies(location, mob.force);
            if enemies.is_empty() {
                continue;
            }

            // Attack
            let location = enemies
                .into_iter()
                .sorted_by(|&a, &b| {
                    match Ord::cmp(&self.unit_at(a).health, &self.unit_at(b).health) {
                        Ordering::Equal => Self::reading_order(a, b),
                        ord => ord,
                    }
                })
                .next()
                .unwrap();
            let target = self.unit_at(location);
            let damage = if mob.force == Force::Elf {
                3 + elfbuff
            } else {
                3
            };
            let after = target.take_damage(damage);
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

#[cfg(test)]
mod game_step {
    use super::*;

    #[test]
    fn move_example() {
        let mut game = generate(include_str!("day15_example_move.txt"));
        game.step(0);
        game.soft_reset();
        assert_eq!(game, generate(include_str!("day15_example_move_2.txt")))
    }
}

#[aoc_generator(day15)]
fn generate(input: &str) -> Game {
    input.parse().unwrap()
}

#[aoc(day15, part1)]
fn solve(game: &Game) -> usize {
    // println!("{}", game);
    score(game)
}

fn score(game: &Game) -> usize {
    let mut game = (*game).clone();
    while !game.is_over() {
        game.step(0);
        // println!("{}", game);
    }
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

#[aoc(day15, part2)]
fn solve2(game: &Game) -> usize {
    elves_no_losses(game)
}

fn elves_no_losses(game: &Game) -> usize {
    let starting_elves = game.elves();
    'buffing: for buff in 1..255 {
        let mut game = (*game).clone();
        while !game.is_over() {
            game.step(buff);
            if game.elves() != starting_elves {
                continue 'buffing;
            }
        }
        return game.score();
    }
    unreachable!();
}

#[cfg(test)]
mod elves_no_losses {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            elves_no_losses(&generate(include_str!("day15_example1.txt"))),
            4988
        );
    }
}
