use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

fn build_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (from, rest) = line
            .split_once(':')
            .expect("each line should contain a ':'");

        let from = from.trim().to_string();

        let targets: Vec<String> = rest
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();

        graph.insert(from, targets);
    }

    graph
}


fn part_one(input: &str) -> i64 {
    let graph = build_graph(input);
    let mut memo: HashMap<String, i64> = HashMap::new();
    count_paths_simple("you", &graph, &mut memo)
}

fn count_paths_simple(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if node == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let neighbors = match graph.get(node) {
        Some(v) => v,
        None => {
            memo.insert(node.to_string(), 0);
            return 0;
        }
    };

    let mut total = 0i64;
    for next in neighbors {
        total += count_paths_simple(next, graph, memo);
    }

    memo.insert(node.to_string(), total);
    total
}


fn part_two(input: &str) -> i64 {
    let graph = build_graph(input);
    let mut memo: HashMap<(String, u8), i64> = HashMap::new();

    count_paths_with_required("svr", 0, &graph, &mut memo)
}

fn count_paths_with_required(
    node: &str,
    mask: u8,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, u8), i64>,
) -> i64 {
    let mut new_mask = mask;
    if node == "dac" {
        new_mask |= 0b01;
    }
    if node == "fft" {
        new_mask |= 0b10;
    }

    let key = (node.to_string(), new_mask);

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if node == "out" {
        let result = if new_mask == 0b11 { 1 } else { 0 };
        memo.insert(key, result);
        return result;
    }

    let neighbors = match graph.get(node) {
        Some(v) => v,
        None => {
            memo.insert(key, 0);
            return 0;
        }
    };

    let mut total = 0i64;
    for next in neighbors {
        total += count_paths_with_required(next, new_mask, graph, memo);
    }

    memo.insert(key, total);
    total
}