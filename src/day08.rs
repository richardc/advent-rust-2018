type Value = usize;

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<Value>,
}

impl Node {
    fn sum_metadata(&self) -> Value {
        self.children
            .iter()
            .map(|c| c.sum_metadata())
            .sum::<Value>()
            + self.metadata.iter().sum::<Value>()
    }
}

fn parse_node(values: &[Value]) -> (Node, usize) {
    let mut offset = 2;
    let mut children = vec![];
    for _ in 0..values[0] {
        let (kid, size) = parse_node(&values[offset..]);
        children.push(kid);
        offset += size;
    }
    let mut metadata = vec![];
    for _ in 0..values[1] {
        metadata.push(values[offset]);
        offset += 1;
    }

    (Node { children, metadata }, offset)
}

#[aoc_generator(day8)]
fn generate(input: &str) -> Node {
    let values: Vec<Value> = input
        .split_ascii_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    let (node, _size) = parse_node(&values);
    node
}

#[aoc(day8, part1)]
fn solve(root: &Node) -> Value {
    root.sum_metadata()
}

#[cfg(test)]
#[test]
fn test_solve() {
    assert_eq!(solve(&generate("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")), 138)
}
