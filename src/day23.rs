use std::cmp::Ordering;

use itertools::{iproduct, Itertools, MinMaxResult};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl std::str::FromStr for Point3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y, z]: [&str; 3] = s[..s.len() - 1]
            .strip_prefix("pos=<")
            .unwrap()
            .split(',')
            .collect_vec()
            .try_into()
            .unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

impl Point3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance_to_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }

    fn distance(&self, other: &Point3) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

impl std::ops::Add<i32> for Point3 {
    type Output = Point3;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::Sub<i32> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Nanobot {
    position: Point3,
    radius: u32,
}

impl std::str::FromStr for Nanobot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (point, radius) = s.split_once(", r=").unwrap();
        Ok(Self {
            position: point.parse()?,
            radius: radius.parse()?,
        })
    }
}

#[derive(Default)]
struct Swarm {
    bots: Vec<Nanobot>,
}

impl std::str::FromStr for Swarm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bots: s.lines().map(|l| l.parse().unwrap()).collect_vec(),
        })
    }
}

impl Swarm {
    fn connected(&self, bot: &Nanobot) -> usize {
        self.bots
            .iter()
            .filter(|b| b.position.distance(&bot.position) <= bot.radius)
            .count()
    }

    fn largest_connected(&self) -> usize {
        let bot = self.bots.iter().max_by_key(|b| b.radius).unwrap();
        self.connected(bot)
    }

    fn closest_cluster(&self) -> u32 {
        let MinMaxResult::MinMax(minx, maxx) = self.bots.iter().map(|b| b.position.x).minmax() else { unreachable!() };
        let MinMaxResult::MinMax(miny, maxy) = self.bots.iter().map(|b| b.position.y).minmax() else { unreachable!() };
        let MinMaxResult::MinMax(minz, maxz) = self.bots.iter().map(|b| b.position.z).minmax() else { unreachable!() };

        let mut min = Point3::new(minx, miny, minz);
        let mut max = Point3::new(maxx, maxy, maxz);
        let mut gridsize = max.x - min.x;
        let mut best = Point3::default();

        while gridsize > 0 {
            let mut max_count = 0;
            for (x, y, z) in iproduct!(
                (min.x..=max.x).step_by(gridsize as usize),
                (min.y..=max.y).step_by(gridsize as usize),
                (min.z..=max.z).step_by(gridsize as usize)
            ) {
                let point = Point3 { x, y, z };
                let count = self
                    .bots
                    .iter()
                    .filter(|b| (b.position.distance(&point) as i32 - b.radius as i32) < gridsize)
                    .count();

                match Ord::cmp(&count, &max_count) {
                    Ordering::Greater => {
                        max_count = count;
                        best = point;
                    }
                    Ordering::Equal if point.distance_to_origin() < best.distance_to_origin() => {
                        best = point;
                    }
                    _ => {}
                }
            }

            min = best - gridsize;
            max = best + gridsize;
            gridsize /= 2;
        }
        best.distance_to_origin()
    }
}

#[aoc_generator(day23)]
fn generate(input: &str) -> Swarm {
    input.parse().unwrap()
}

#[aoc(day23, part1)]
fn solve(swarm: &Swarm) -> usize {
    swarm.largest_connected()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day23_example.txt"))), 7)
}

#[aoc(day23, part2)]
fn solve2(swarm: &Swarm) -> u32 {
    swarm.closest_cluster()
}

#[cfg(test)]
#[test]
fn test_solve2() {
    assert_eq!(solve2(&generate(include_str!("day23_example2.txt"))), 36)
}
