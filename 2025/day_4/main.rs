use std::collections::VecDeque;

const INPUT: &str = include_str!("./input.txt");
const NEIGHBORS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn main() {
    let grid = parse_grid(INPUT);
    let part_one_result: i64 = part_one(&grid);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(&grid);
    println!("Part Two: {part_two_result}");
}

fn has_fewer_than_four_adjacent(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut adjacent_count = 0;
    for (dr, dc) in NEIGHBORS.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;
        if new_row < 0 || new_row >= grid.len() as i32 || new_col < 0 || new_col >= grid[row].len() as i32 {
            continue;
        }
        if grid[new_row as usize][new_col as usize] == '@' {
            adjacent_count += 1;
        }
    }
    adjacent_count < 4
}

fn part_one(grid: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '@' {
                if has_fewer_than_four_adjacent(grid, row, col) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part_two(grid: &Vec<Vec<char>>) -> i64 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut degree = vec![vec![0i32; cols]; rows];
    let mut removed = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }
            let mut count = 0;
            for (dr, dc) in NEIGHBORS.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                    continue;
                }
                if grid[nr as usize][nc as usize] == '@' {
                    count += 1;
                }
            }
            degree[r][c] = count;
        }
    }

    let mut queue = VecDeque::new();
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' && degree[r][c] < 4 {
                queue.push_back((r, c));
            }
        }
    }

    let mut removed_count: i64 = 0;

    while let Some((r, c)) = queue.pop_front() {
        if removed[r][c] || grid[r][c] != '@' {
            continue;
        }

        removed[r][c] = true;
        removed_count += 1;

        for (dr, dc) in NEIGHBORS.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            if grid[nr][nc] == '@' && !removed[nr][nc] {
                degree[nr][nc] -= 1;
                if degree[nr][nc] < 4 {
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    removed_count
}

