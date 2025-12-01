const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> i64 {
    let mut position: i64 = 50;
    let mut zero_hits: i64 = 0;

    for rotation in input.lines() {
        let (dir, num) = rotation.split_at(1);
        let value: i64 = num.parse().unwrap();

        let delta = if dir == "L" {
            -value
        } else {
            value
        };

        position = (position + delta).rem_euclid(100);

        if position == 0 {
            zero_hits += 1;
        }
    }
    zero_hits
}

fn part_two(input: &str) -> i64 {
    let mut position: i64 = 50;
    let mut zero_hits: i64 = 0;

    for rotation in input.lines() {
        let (dir, num) = rotation.split_at(1);
        let amount: i64 = num.parse().unwrap();

        let step = if dir == "L" { -1 } else { 1 };

        for _ in 0..amount {
            position = (position + step).rem_euclid(100);

            if position == 0 {
                zero_hits += 1;
            }
        }
    }

    zero_hits
}
