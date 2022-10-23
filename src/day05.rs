#[aoc(day5, part1, original)]
fn solve(s: &str) -> usize {
    stable_length_original(s)
}

#[aoc(day5, part1, retain)]
fn solve_retain(s: &str) -> usize {
    stable_length_retain(s)
}

fn stable_length_original(s: &str) -> usize {
    let mut elements = s.as_bytes().to_vec();
    loop {
        let before = elements.len();
        for i in 0..elements.len() - 1 {
            if elements[i].to_ascii_lowercase() == elements[i + 1].to_ascii_lowercase()
                && elements[i] != elements[i + 1]
            {
                elements.remove(i);
                elements.remove(i);
                break;
            }
        }
        if elements.is_empty() || before == elements.len() {
            return elements.len();
        }
    }
}

fn stable_length_retain(s: &str) -> usize {
    let mut elements = s.as_bytes().to_vec();
    loop {
        let mut i = 0;
        while i < elements.len() - 1 {
            if elements[i].to_ascii_lowercase() == elements[i + 1].to_ascii_lowercase()
                && elements[i] != elements[i + 1]
            {
                elements[i] = 0;
                elements[i + 1] = 0;
                i += 1;
            }
            i += 1;
        }
        let before = elements.len();
        elements.retain(|&b| b != 0);
        if elements.is_empty() || elements.len() == before {
            return elements.len();
        }
    }
}

#[test_case("aA" => 0)]
#[test_case("abBA" => 0)]
#[test_case("abAB" => 4)]
#[test_case("aabAAB" => 6)]
#[test_case("dabAcCaCBAcCcaDA" => 10)]
#[cfg(test)]
fn _stable_length(s: &str) -> usize {
    let original = stable_length_original(s);
    let retain = stable_length_retain(s);
    assert_eq!(original, retain);
    retain
}

#[aoc(day5, part2)]
fn solve2(s: &str) -> usize {
    ('a'..='z')
        .map(|c| {
            let mut test: Vec<u8> = s.as_bytes().to_vec();
            test.retain(|b| b.to_ascii_lowercase() != c as u8);
            stable_length_retain(std::str::from_utf8(&test).unwrap())
        })
        .min()
        .unwrap()
}

#[test_case("dabAcCaCBAcCcaDA" => 4)]
#[cfg(test)]
fn test_solve2(s: &str) -> usize {
    solve2(s)
}
