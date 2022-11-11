use std::collections::HashMap;

#[derive(Default)]
struct Walker {
    x: i32,
    y: i32,
    steps: usize,
    visited: HashMap<(i32, i32), usize>,
}

impl Walker {
    fn step(&mut self, x_delta: i32, y_delta: i32) {
        self.x += x_delta;
        self.y += y_delta;
        self.steps += 1;

        self.visited
            .entry((self.x, self.y))
            .and_modify(|e| *e = std::cmp::min(*e, self.steps))
            .or_insert(self.steps);
    }

    fn tour(&mut self, path: &str) {
        let mut stack = vec![];
        for char in path.chars() {
            match char {
                'N' => self.step(0, 1),
                'S' => self.step(0, -1),
                'E' => self.step(1, 0),
                'W' => self.step(-1, 0),
                '(' => stack.push((self.x, self.y, self.steps)),
                ')' => (self.x, self.y, _) = stack.pop().unwrap(),
                '|' => (self.x, self.y, self.steps) = stack[stack.len() - 1],
                _ => {}
            }
        }
    }

    fn furthest(&self) -> usize {
        *self.visited.values().max().unwrap()
    }

    fn far_away(&self, limit: usize) -> usize {
        self.visited.values().filter(|&s| *s >= limit).count()
    }
}

#[aoc(day20, part1)]
fn solve(s: &str) -> usize {
    let mut walker = Walker::default();
    walker.tour(s);
    walker.furthest()
}

#[aoc(day20, part2)]
fn solve2(s: &str) -> usize {
    let mut walker = Walker::default();
    walker.tour(s);
    walker.far_away(1_000)
}

#[cfg(test)]
#[test_case("^WNE$" => 3)]
#[test_case("^ENWWW(NEEE|SSE(EE|N))$" => 10)]
#[test_case("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$" => 18)]
#[test_case("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$" => 23)]
#[test_case("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$" => 31)]
fn _solve(s: &str) -> usize {
    solve(s)
}
