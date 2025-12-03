const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result = part_one(INPUT);
    println!("Part One (2 digits): {part_one_result}");

    let part_two_result = part_two(INPUT);
    println!("Part Two (12 digits): {part_two_result}");
}


fn part_one(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| best_bank_joltage_2(line.trim()))
        .sum()
}

fn best_bank_joltage_2(line: &str) -> i64 {
    let bytes = line.as_bytes();

    let mut max_left = (bytes[0] - b'0') as i64;
    let mut best = 0i64;

    for &b in &bytes[1..] {
        let d = (b - b'0') as i64;

        let candidate = max_left * 10 + d;
        if candidate > best {
            best = candidate;
        }

        if d > max_left {
            max_left = d;
        }
    }

    best
}


fn part_two(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| best_bank_joltage_k(line.trim(), 12))
        .sum()
}

fn best_bank_joltage_k(line: &str, k: usize) -> i64 {
    let digits = line.as_bytes();
    let n = digits.len();

    if n <= k {
        return digits_to_i64(digits);
    }

    let mut to_remove = n - k;
    let mut stack: Vec<u8> = Vec::with_capacity(n);

    for &d in digits {
        while !stack.is_empty() && to_remove > 0 && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);
    digits_to_i64(&stack)
}

fn digits_to_i64(digits: &[u8]) -> i64 {
    let mut value = 0i64;
    for &b in digits {
        value = value * 10 + ((b - b'0') as i64);
    }
    value
}
