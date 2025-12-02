const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> i64 {
    process_ranges(input, is_invalid)
}

fn is_invalid(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;

    &s[0..half] == &s[half..]
}


fn part_two(input: &str) -> i64 {
    process_ranges(input, is_invalid_part_two)
}

fn process_ranges(input: &str, check_fn: fn(i64) -> bool) -> i64 {
    let mut invalid_ids: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        for range_str in line.split(',') {
            let range_str = range_str.trim();
            if range_str.is_empty() {
                continue;
            }

            let Some((start_str, end_str)) = range_str.split_once('-') else {
                panic!("Invalid range line: {range_str}");
            };

            let start: i64 = start_str
                .trim()
                .parse()
                .expect("Invalid start number in range");
            let end: i64 = end_str
                .trim()
                .parse()
                .expect("Invalid end number in range");

            for id in start..=end {
                if check_fn(id) {
                    invalid_ids += id;
                }
            }
        }
    }

    invalid_ids
}

fn is_invalid_part_two(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();
    let bytes = s.as_bytes();
    
    for prefix_len in 1..=len / 2 {
        if len % prefix_len == 0 {
            let repetitions = len / prefix_len;
            let mut is_match = true;
            
            for rep in 1..repetitions {
                for i in 0..prefix_len {
                    if bytes[i] != bytes[rep * prefix_len + i] {
                        is_match = false;
                        break;
                    }
                }
                if !is_match {
                    break;
                }
            }
            
            if is_match {
                return true;
            }
        }
    }
    
    false
}