use std::collections::HashMap;

use itertools::{iproduct, Itertools};
use pathfinding::prelude::dijkstra;

enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Tool {
    None,
    Torch,
    Gear,
}

#[derive(Default, Clone)]
struct Cave {
    depth: usize,
    target: (usize, usize),
    geologic: HashMap<(usize, usize), usize>,
}

impl std::str::FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let depth = lines[0].strip_prefix("depth: ").unwrap().parse()?;
        let (x, y) = lines[1]
            .strip_prefix("target: ")
            .unwrap()
            .split_once(',')
            .unwrap();
        Ok(Self {
            depth,
            target: (x.parse()?, y.parse()?),
            ..Default::default()
        })
    }
}

impl Cave {
    fn geologic(&mut self, x: usize, y: usize) -> usize {
        if let Some(value) = self.geologic.get(&(x, y)) {
            return *value;
        }
        let value = match (x, y) {
            (0, 0) => 0,
            (x, y) if (x, y) == self.target => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => self.erosion(x - 1, y) * self.erosion(x, y - 1),
        };
        self.geologic.insert((x, y), value);
        value
    }

    fn erosion(&mut self, x: usize, y: usize) -> usize {
        (self.geologic(x, y) + self.depth) % 20183
    }

    fn region(&mut self, x: usize, y: usize) -> usize {
        self.erosion(x, y) % 3
    }

    fn terrain(&mut self, x: usize, y: usize) -> Terrain {
        use Terrain::*;
        match self.region(x, y) {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!(),
        }
    }

    fn risk_level(&mut self) -> usize {
        iproduct!(0..=self.target.0, 0..=self.target.1)
            .map(|(x, y)| self.region(x, y))
            .sum()
    }

    fn legal_tool(terrain: Terrain, tool: Tool) -> bool {
        use Terrain::*;
        use Tool::*;
        matches!(
            (terrain, tool),
            (Rocky, Torch)
                | (Rocky, Gear)
                | (Wet, None)
                | (Wet, Gear)
                | (Narrow, None)
                | (Narrow, Torch)
        )
    }

    fn neighbours(&mut self, x: usize, y: usize, tool: Tool) -> Vec<((usize, usize, Tool), usize)> {
        let terrain = self.terrain(x, y);
        let mut neighbours = vec![];
        use Terrain::*;
        use Tool::*;
        // swap tool
        match (terrain, tool) {
            (Rocky, Torch) => neighbours.push(((x, y, Gear), 7)),
            (Rocky, Gear) => neighbours.push(((x, y, Torch), 7)),
            (Wet, None) => neighbours.push(((x, y, Gear), 7)),
            (Wet, Gear) => neighbours.push(((x, y, None), 7)),
            (Narrow, None) => neighbours.push(((x, y, Torch), 7)),
            (Narrow, Torch) => neighbours.push(((x, y, None), 7)),
            (_, _) => (),
        }

        // given current tool
        if x > 0 && Self::legal_tool(self.terrain(x - 1, y), tool) {
            neighbours.push(((x - 1, y, tool), 1))
        }
        if Self::legal_tool(self.terrain(x + 1, y), tool) {
            neighbours.push(((x + 1, y, tool), 1))
        }
        if y > 0 && Self::legal_tool(self.terrain(x, y - 1), tool) {
            neighbours.push(((x, y - 1, tool), 1))
        }
        if Self::legal_tool(self.terrain(x, y + 1), tool) {
            neighbours.push(((x, y + 1, tool), 1))
        }
        neighbours
    }

    fn rescue_time(&mut self) -> usize {
        let target = (self.target.0, self.target.1, Tool::Torch);
        let (_path, cost) = dijkstra(
            &(0, 0, Tool::Torch),
            |&(x, y, tool)| self.neighbours(x, y, tool),
            |&node| target == node,
        )
        .unwrap();
        cost
    }
}

#[aoc_generator(day22)]
fn generate(input: &str) -> Cave {
    input.parse().unwrap()
}

#[aoc(day22, part1)]
fn solve(cave: &Cave) -> usize {
    (*cave).clone().risk_level()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day22_example.txt"))), 114)
}

#[aoc(day22, part2)]
fn solve2(cave: &Cave) -> usize {
    (*cave).clone().rescue_time()
}

#[cfg(test)]
#[test]
fn test_solve2() {
    assert_eq!(solve2(&generate(include_str!("day22_example.txt"))), 45)
}
