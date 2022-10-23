use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum What {
    Guard(usize),
    Sleep,
    Wakes,
}

impl std::str::FromStr for What {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split_ascii_whitespace().collect_vec();
        match chunks[0] {
            "falls" => Ok(What::Sleep),
            "wakes" => Ok(What::Wakes),
            _ => Ok(What::Guard(chunks[1][1..].parse()?)),
        }
    }
}

#[derive(Debug)]
struct Event {
    minute: usize,
    what: What,
}

impl std::str::FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = s.split("] ").collect_vec();
        let time = chunks[0].split(':').collect_vec();
        Ok(Event {
            minute: time[1].parse()?,
            what: chunks[1].parse()?,
        })
    }
}

#[derive(Debug)]
struct Guard {
    minutes: [usize; 60],
}

impl Guard {
    fn new() -> Self {
        Self { minutes: [0; 60] }
    }

    fn record_sleep(&mut self, start: usize, end: usize) {
        for i in start..end {
            self.minutes[i] += 1;
        }
    }

    fn time_sleeping(&self) -> usize {
        self.minutes.iter().sum()
    }

    fn max_minute(&self) -> usize {
        *self.minutes.iter().max().unwrap()
    }

    fn common_minute(&self) -> usize {
        let max = self.max_minute();
        self.minutes.iter().position(|&v| v == max).unwrap()
    }
}

#[aoc_generator(day4)]
fn generate(input: &str) -> Vec<Event> {
    input.lines().sorted().map(|l| l.parse().unwrap()).collect()
}

fn exploit_weakest_guard(
    events: &[Event],
    weakest: fn(&Guard, &Guard) -> std::cmp::Ordering,
) -> usize {
    let mut guards: HashMap<usize, Guard> = HashMap::new();
    let mut guard = 0;
    let mut start = 0;
    for event in events {
        match event.what {
            What::Guard(id) => guard = id,
            What::Sleep => start = event.minute,
            What::Wakes => {
                guards
                    .entry(guard)
                    .and_modify(|g| g.record_sleep(start, event.minute))
                    .or_insert_with(|| {
                        let mut g = Guard::new();
                        g.record_sleep(start, event.minute);
                        g
                    });
            }
        }
    }

    guards
        .iter()
        .sorted_by(|(_, av), (_, bv)| weakest(av, bv))
        .map(|(&k, v)| k * v.common_minute() as usize)
        .next()
        .unwrap()
}

#[aoc(day4, part1)]
fn solve(events: &[Event]) -> usize {
    exploit_weakest_guard(events, |a, b| {
        Ord::cmp(&b.time_sleeping(), &a.time_sleeping())
    })
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate(include_str!("day04_example.txt"))), 240)
}

#[aoc(day4, part2)]
fn solve2(events: &[Event]) -> usize {
    exploit_weakest_guard(events, |a, b| Ord::cmp(&b.max_minute(), &a.max_minute()))
}

#[cfg(test)]
#[test]
fn test_solve2() {
    assert_eq!(solve2(&generate(include_str!("day04_example.txt"))), 4455)
}
