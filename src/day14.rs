use itertools::Itertools;

#[aoc(day14, part1)]
fn solve(input: &str) -> String {
    score_next_ten(input.parse().unwrap())
}

#[allow(dead_code)] // sometimes used
fn display(one: usize, two: usize, digits: &[u8]) {
    for (i, digit) in digits.iter().enumerate() {
        if i == one {
            print!("(");
        }
        if i == two {
            print!("[");
        }
        print!("{}", digit);
        if i == two {
            print!("]");
        }
        if i == one {
            print!(")");
        }
        print!(" ");
    }
    println!();
}

fn score_next_ten(count: usize) -> String {
    let mut digits = vec![3_u8, 7];
    let mut one = 0;
    let mut two = 1;

    for _ in 0..count + 10 {
        let sum = digits[one as usize] + digits[two as usize];
        digits.append(
            &mut format!("{sum}")
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec(),
        );

        one += 1 + digits[one] as usize;
        one %= digits.len();
        two += 1 + digits[two] as usize;
        two %= digits.len();

        // display(one, two, &digits);
    }

    String::from_iter(
        digits[count..count + 10]
            .iter()
            .map(|&b| (b'0' + b) as char),
    )
}

#[test_case(5 => "0124515891")]
#[test_case(9 => "5158916779")]
#[test_case(18 => "9251071085")]
#[test_case(2018 => "5941429882")]
#[cfg(test)]
fn _next_ten(count: usize) -> String {
    score_next_ten(count)
}

fn find_run(needle: &[u8]) -> usize {
    let mut digits = vec![3_u8, 7];
    let mut one = 0;
    let mut two = 1;

    loop {
        let sum = digits[one as usize] + digits[two as usize];
        digits.append(
            &mut format!("{sum}")
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec(),
        );

        one += 1 + digits[one] as usize;
        one %= digits.len();
        two += 1 + digits[two] as usize;
        two %= digits.len();

        if digits.len() > needle.len() {
            for start in
                (digits.len() - needle.len()).saturating_sub(2)..digits.len() - needle.len()
            {
                if &digits[start..start + needle.len()] == needle {
                    return start;
                }
            }
        }
    }
}

#[test_case("01245" => 5)]
#[test_case("51589" => 9)]
#[test_case("92510" => 18)]
#[test_case("59414" => 2018)]
#[cfg(test)]
fn _find_run(s: &str) -> usize {
    find_run(
        &s.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect_vec(),
    )
}

#[aoc(day14, part2)]
fn solve2(input: &str) -> usize {
    find_run(
        &input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect_vec(),
    )
}
