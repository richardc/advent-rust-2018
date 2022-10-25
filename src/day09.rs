use std::collections::VecDeque;

#[aoc_generator(day9)]
fn generate(s: &str) -> [usize; 2] {
    let chunks: Vec<_> = s.split_ascii_whitespace().collect();
    let players = chunks[0].parse().unwrap();
    let last_marble = chunks[6].parse().unwrap();

    [players, last_marble]
}

#[aoc(day9, part1)]
fn solve(&[players, last_marble]: &[usize; 2]) -> usize {
    winning_score(players, last_marble)
}

#[aoc(day9, part2)]
fn solve2(&[players, last_marble]: &[usize; 2]) -> usize {
    winning_score(players, last_marble * 100)
}

fn winning_score(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    let mut circle = VecDeque::from([0]);
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            scores[marble % players] += marble + circle.pop_back().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }
    *scores.iter().max().unwrap()
}

#[test_case(9, 25 => 32)]
#[test_case(10, 1618 => 8317)]
#[test_case(13, 7999 => 146373)]
#[test_case(17, 1104 => 2764)]
#[test_case(21, 6111 => 54718)]
#[test_case(30, 5807 => 37305)]
#[cfg(test)]
fn _score(players: usize, last_marble: usize) -> usize {
    winning_score(players, last_marble)
}
