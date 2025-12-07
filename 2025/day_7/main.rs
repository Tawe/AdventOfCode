const INPUT: &str = include_str!("./input.txt");

use std::collections::HashSet;

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut start_row = 0;
    let mut start_col = 0;

    'outer: for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                start_row = r;
                start_col = c;
                break 'outer;
            }
        }
    }

    let mut active: HashSet<usize> = HashSet::new();
    active.insert(start_col);

    let mut splits: i64 = 0;

    for r in (start_row + 1)..height {
        let mut next_active: HashSet<usize> = HashSet::new();

        for &c in &active {
            match grid[r][c] {
                '.' | 'S' => {
                    next_active.insert(c);
                }
                '^' => {
                    splits += 1;
                    if c > 0 {
                        next_active.insert(c - 1);
                    }
                    if c + 1 < width {
                        next_active.insert(c + 1);
                    }
                }
                _ => {
                    next_active.insert(c);
                }
            }
        }

        active = next_active;
        if active.is_empty() {
            break;
        }
    }

    splits
}


fn part_two(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();
    let mut s_row = 0;
    let mut s_col = 0;
    
    'outer: for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                s_row = r;
                s_col = c;
                break 'outer;
            }
        }
    }
    let mut curr = vec![0u128; width];
    curr[s_col] = 1; 

    for r in s_row..(height - 1) {
        let mut next = vec![0u128; width];

        for c in 0..width {
            let count = curr[c];
            if count == 0 {
                continue;
            }

            let ch = grid[r + 1][c];
            match ch {
                '.' | 'S' => {
                    next[c] += count;
                }
                '^' => {
                    if c > 0 {
                        next[c - 1] += count;
                    }
                    if c + 1 < width {
                        next[c + 1] += count;
                    }
                }
                _ => {
                    next[c] += count;
                }
            }
        }

        curr = next;
    }
    let total: u128 = curr.iter().sum();

    total.try_into().unwrap()
}
