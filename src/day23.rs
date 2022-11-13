use itertools::Itertools;

#[derive(Default)]
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

#[derive(Default)]
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

impl Nanobot {
    fn distance(&self, other: &Nanobot) -> u32 {
        self.position.x.abs_diff(other.position.x)
            + self.position.y.abs_diff(other.position.y)
            + self.position.z.abs_diff(other.position.z)
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
            .filter(|b| b.distance(bot) <= bot.radius)
            .count()
    }

    fn largest_connected(&self) -> usize {
        let bot = self.bots.iter().max_by_key(|b| b.radius).unwrap();
        self.connected(bot)
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
