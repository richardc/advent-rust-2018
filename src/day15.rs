use itertools::{Either, Itertools};
use pathfinding::prelude::Matrix;

type Health = u8;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Goblin(Health),
    Elf(Health),
}

impl Cell {
    fn new(c: char) -> Self {
        use Cell::*;
        match c {
            '#' => Wall,
            'G' => Goblin(200),
            'E' => Elf(200),
            _ => Empty,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        write!(
            f,
            "{}",
            match &self {
                Empty => '.',
                Wall => '#',
                Goblin(_) => 'G',
                Elf(_) => 'E',
            }
        )
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Empty => write!(f, "."),
            Wall => write!(f, "#"),
            Goblin(health) => write!(f, "G({})", health),
            Elf(health) => write!(f, "E({})", health),
        }
    }
}

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
        for row in self.map.iter() {
            let mut mobs = vec![];
            for col in row {
                write!(f, "{}", col)?;
                match col {
                    Cell::Goblin(_) | Cell::Elf(_) => mobs.push(col),
                    _ => {}
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
    fn winning_team(&self) -> Vec<Health> {
        let (elves, goblins): (Vec<_>, Vec<_>) = self
            .map
            .values()
            .filter(|c| matches!(c, Cell::Elf(_) | Cell::Goblin(_)))
            .partition_map(|&cell| match cell {
                Cell::Elf(health) => Either::Left(health),
                Cell::Goblin(health) => Either::Right(health),
                _ => unreachable!(),
            });
        if goblins.is_empty() {
            return elves;
        }
        goblins
    }

    fn score(&self) -> usize {
        self.winning_team()
            .iter()
            .map(|&h| h as usize)
            .sum::<usize>()
            * self.round
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
