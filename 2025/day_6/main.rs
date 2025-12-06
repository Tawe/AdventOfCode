const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Problem {
    op: char, 
    nums: Vec<i64>,
}

#[derive(Debug)]
struct Worksheet {
    grid: Vec<Vec<u8>>,
    spans: Vec<(usize, usize)>,
    op_row: usize,
}

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

fn part_one(input: &str) -> i64 {
    let ws = build_worksheet(input);
    let problems = problems_by_rows(&ws);
    problems.iter().map(eval_problem).sum()
}


fn part_two(input: &str) -> i64 {
    let ws = build_worksheet(input);
    let problems = problems_by_cols(&ws);
    problems.iter().map(eval_problem).sum()
}


fn build_worksheet(input: &str) -> Worksheet {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    let height = lines.len();
    assert!(height >= 2, "need at least one row of numbers and one row of operators");

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            let mut bytes = l.as_bytes().to_vec();
            bytes.resize(width, b' ');
            bytes
        })
        .collect();

    let op_row = height - 1;

    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut col = 0;

    while col < width {
        let mut is_gap = true;
        for row in 0..height {
            if grid[row][col] != b' ' {
                is_gap = false;
                break;
            }
        }

        if is_gap {
            col += 1;
            continue;
        }

        let start = col;
        while col < width {
            let mut all_spaces = true;
            for row in 0..height {
                if grid[row][col] != b' ' {
                    all_spaces = false;
                    break;
                }
            }
            if all_spaces {
                break;
            }
            col += 1;
        }
        let end = col; 
        spans.push((start, end));
    }

    Worksheet { grid, spans, op_row }
}

fn problems_by_rows(ws: &Worksheet) -> Vec<Problem> {
    use std::str;

    let mut problems = Vec::with_capacity(ws.spans.len());
    let op_row = ws.op_row;

    for &(start, end) in &ws.spans {
        let mut op = None;
        for c in start..end {
            let ch = ws.grid[op_row][c] as char;
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("no operator found in span (part 1)");

        let mut nums = Vec::new();
        for row in 0..op_row {
            let slice = &ws.grid[row][start..end];
            let s = str::from_utf8(slice).unwrap().trim();
            if !s.is_empty() {
                nums.push(s.parse::<i64>().expect("invalid number"));
            }
        }

        problems.push(Problem { op, nums });
    }

    problems
}

fn problems_by_cols(ws: &Worksheet) -> Vec<Problem> {
    let mut problems = Vec::with_capacity(ws.spans.len());
    let op_row = ws.op_row;

    for &(start, end) in &ws.spans {
        let mut op = None;
        for c in start..end {
            let ch = ws.grid[op_row][c] as char;
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("no operator found in span (part 2)");
        let mut nums = Vec::new();

        for col in (start..end).rev() {
            let mut digits = String::new();
            for row in 0..op_row {
                digits.push(ws.grid[row][col] as char);
            }

            let s = digits.trim();
            if !s.is_empty() {
                nums.push(s.parse::<i64>().expect("invalid column-number"));
            }
        }

        problems.push(Problem { op, nums });
    }

    problems
}

fn eval_problem(p: &Problem) -> i64 {
    match p.op {
        '+' => p.nums.iter().copied().sum(),
        '*' => p.nums.iter().copied().product(),
        other => panic!("unexpected operator: {other}"),
    }
}
