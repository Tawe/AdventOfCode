const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT).try_into().unwrap();
    println!("Part Two: {part_two_result}");
}

fn parse_range(range: &str) -> Option<(u128, u128)> {
    let (start_str, end_str) = range.trim().split_once('-')?;
    let start = start_str.trim().parse::<u128>().ok()?;
    let end = end_str.trim().parse::<u128>().ok()?;
    Some((start, end))
}

fn part_one(input: &str) -> i64 {
    let (ranges_block, ids_block) = input
        .split_once("\n\n")
        .expect("expected two blocks separated by a blank line");

    let ranges: Vec<(u128, u128)> = ranges_block
        .lines()
        .filter_map(|line| parse_range(line))
        .collect();

    let mut count: i64 = 0;

    for id_str in ids_block.lines() {
        let id: u128 = match id_str.trim().parse() {
            Ok(v) => v,
            Err(_) => continue, 
        };

        let in_any_range = ranges
            .iter()
            .any(|(start, end)| id >= *start && id <= *end);

        if in_any_range {
            count += 1;
        }
    }

    count
}

fn part_two(input: &str) -> u128 {
    let (ranges_block, _ids_block) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u128, u128)> = ranges_block
        .lines()
        .filter_map(parse_range)
        .collect();

    ranges.sort_by_key(|(start, _)| *start);

    let mut total: u128 = 0;

    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            total += current.1 - current.0 + 1;
            current = (start, end);
        }
    }

    total += current.1 - current.0 + 1;

    total
}
