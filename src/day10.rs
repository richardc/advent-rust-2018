use itertools::{Itertools, MinMaxResult};
use lazy_static::lazy_static;
use pathfinding::grid::Grid;
use regex::Regex;

type Number = i64;

#[derive(Debug)]
struct Point {
    x: Number,
    y: Number,
}

#[derive(Debug)]
struct Observation {
    position: Point,
    vector: Point,
}

impl std::str::FromStr for Observation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"position=<(.*?), (.*?)> velocity=<(.*?), (.*?)>").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        Ok(Observation {
            position: Point {
                x: caps.get(1).unwrap().as_str().trim().parse()?,
                y: caps.get(2).unwrap().as_str().trim().parse()?,
            },
            vector: Point {
                x: caps.get(3).unwrap().as_str().trim().parse()?,
                y: caps.get(4).unwrap().as_str().trim().parse()?,
            },
        })
    }
}

fn points_at(time: Number, sky: &[Observation]) -> Vec<Point> {
    sky.iter()
        .map(|o| Point {
            x: o.position.x + time * o.vector.x,
            y: o.position.y + time * o.vector.y,
        })
        .collect()
}

fn area(points: &[Point]) -> Number {
    let (min_x, max_x) = match points.iter().map(|p| p.x).minmax() {
        MinMaxResult::MinMax(min_x, max_x) => (min_x, max_x),
        _ => (0, 0),
    };
    let (min_y, max_y) = match points.iter().map(|p| p.y).minmax() {
        MinMaxResult::MinMax(min_y, max_y) => (min_y, max_y),
        _ => (0, 0),
    };
    (min_x.abs_diff(max_x) * min_y.abs_diff(max_y))
        .try_into()
        .unwrap()
}

fn smallest_area(sky: &[Observation]) -> (Number, Vec<Point>) {
    (1..15_000)
        .into_iter()
        .map(|time| (time, points_at(time, sky)))
        .sorted_by(|a, b| Ord::cmp(&area(&a.1), &area(&b.1)))
        .next()
        .unwrap()
}

fn render(points: &[Point]) -> String {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let grid = Grid::from_iter(
        points
            .iter()
            .map(|p| ((p.x - min_x) as usize, (p.y - min_y) as usize)),
    );
    format!("{:?}", grid).replace('.', " ")
}

#[cfg(test)]
#[test]
fn test_smallest_area() {
    let (time, _) = smallest_area(&generate(include_str!("day10_example.txt")));
    assert_eq!(time, 3);
}

#[aoc_generator(day10)]
fn generate(input: &str) -> Vec<Observation> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn solve(seen: &[Observation]) -> String {
    let (time, points) = smallest_area(seen);
    format!("At second {}\n{}", time, render(&points))
}
