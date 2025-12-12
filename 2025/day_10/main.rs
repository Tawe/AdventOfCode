use good_lp::{
    variable, Expression, ProblemVariables, SolverModel, Solution,
    microlp,
};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

#[derive(Debug)]
struct Machine {
    pattern: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    target: Vec<i64>,
}

fn parse_machine(line: &str) -> Machine {
    let s = line.trim();
    if s.is_empty() {
        return Machine {
            pattern: vec![],
            buttons: vec![],
            target: vec![],
        };
    }

    let lb = s.find('[').expect("no [");
    let rb = s[lb + 1..].find(']').expect("no ]") + lb + 1;
    let pattern_str = &s[lb + 1..rb];
    let pattern: Vec<bool> = pattern_str.chars().map(|c| c == '#').collect();


    let mut rest = &s[rb + 1..];

    let mut buttons: Vec<Vec<usize>> = Vec::new();
    loop {
        if let Some(start) = rest.find('(') {
            let after = &rest[start + 1..];
            if let Some(end_rel) = after.find(')') {
                let end = start + 1 + end_rel;
                let inside = &rest[start + 1..end];
                let indices: Vec<usize> = inside
                    .split(',')
                    .filter_map(|t| {
                        let t = t.trim();
                        if t.is_empty() {
                            None
                        } else {
                            Some(t.parse::<usize>().unwrap())
                        }
                    })
                    .collect();
                buttons.push(indices);
                rest = &rest[end + 1..];
                continue;
            }
        }
        break;
    }

    let mut target: Vec<i64> = Vec::new();
    if let Some(lb2) = s.find('{') {
        if let Some(rb2) = s[lb2 + 1..].find('}') {
            let rb2 = lb2 + 1 + rb2;
            let inside = &s[lb2 + 1..rb2];
            target = inside
                .split(',')
                .filter_map(|t| {
                    let t = t.trim();
                    if t.is_empty() {
                        None
                    } else {
                        Some(t.parse::<i64>().unwrap())
                    }
                })
                .collect();
        }
    }

    Machine {
        pattern,
        buttons,
        target,
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_machine)
        .collect()
}

fn solve_machine_part1(pattern: &[bool], buttons: &[Vec<usize>]) -> i64 {
    let n = pattern.len();
    let m = buttons.len();

    if n == 0 {
        return 0;
    }
    if m == 0 {
        if pattern.iter().all(|&b| !b) {
            return 0;
        } else {
            panic!("Machine has no buttons but non-zero target pattern");
        }
    }

    let target: Vec<u8> = pattern.iter().map(|&b| if b { 1 } else { 0 }).collect();

    let mut mat = vec![vec![0u8; m + 1]; n];

    for (j, inds) in buttons.iter().enumerate() {
        for &i_light in inds {
            if i_light >= n {
                panic!("button references light index {} but n = {}", i_light, n);
            }
            mat[i_light][j] ^= 1;
        }
    }
    for i in 0..n {
        mat[i][m] = target[i];
    }

    let mut row = 0usize;
    let nrows = n;
    let mut pivot_cols: Vec<usize> = Vec::new();

    for col in 0..m {
        let mut pivot_row = None;
        for r in row..nrows {
            if mat[r][col] == 1 {
                pivot_row = Some(r);
                break;
            }
        }
        let Some(pivot_r) = pivot_row else {
            continue;
        };
        
        mat.swap(row, pivot_r);
        pivot_cols.push(col);

        for r in (row + 1)..nrows {
            if mat[r][col] == 1 {
                for c in col..=m {
                    mat[r][c] ^= mat[row][c];
                }
            }
        }

        row += 1;
        if row == nrows {
            break;
        }
    }

    let rank = pivot_cols.len();

    for r in rank..nrows {
        let all_zero = (0..m).all(|c| mat[r][c] == 0);
        if all_zero && mat[r][m] == 1 {
            panic!("No solution for machine (inconsistent system)");
        }
    }

    for i in (0..rank).rev() {
        let col = pivot_cols[i];
        for r in 0..i {
            if mat[r][col] == 1 {
                for c in col..=m {
                    mat[r][c] ^= mat[i][c];
                }
            }
        }
    }

    let mut is_pivot = vec![false; m];
    for &c in &pivot_cols {
        is_pivot[c] = true;
    }
    let free_cols: Vec<usize> = (0..m).filter(|&c| !is_pivot[c]).collect();
    let k = free_cols.len();

    if k > 25 {
        panic!("Too many free variables ({}); consider optimizing", k);
    }

    let mut best: Option<i64> = None;
    let total_free_masks = 1usize << k;

    for mask in 0..total_free_masks {
        let mut x = vec![0u8; m];

        for (bit_idx, &col) in free_cols.iter().enumerate() {
            if (mask >> bit_idx) & 1 == 1 {
                x[col] = 1;
            }
        }

        for (row_idx, &pivot_col) in pivot_cols.iter().enumerate() {
            let mut val = mat[row_idx][m]; // RHS
            for &free_col in &free_cols {
                if mat[row_idx][free_col] == 1 && x[free_col] == 1 {
                    val ^= 1;
                }
            }
            x[pivot_col] = val;
        }

        let presses = x.iter().map(|&v| v as i64).sum::<i64>();
        if best.map_or(true, |b| presses < b) {
            best = Some(presses);
        }
    }

    best.expect("there should always be at least one solution")
}

fn part_one(input: &str) -> i64 {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|m| solve_machine_part1(&m.pattern, &m.buttons))
        .sum()
}

fn min_presses_jolts(machine: &Machine) -> i64 {
    let num_buttons = machine.buttons.len();
    let num_counters = machine.target.len();

    if num_counters == 0 {
        return 0;
    }

    for (i, &t) in machine.target.iter().enumerate() {
        if t > 0 && !machine.buttons.iter().any(|b| b.contains(&i)) {
            panic!(
                "Unsatisfiable machine: counter {} never changes but target is {}",
                i, t
            );
        }
    }

    let mut vars = ProblemVariables::new();
    let mut button_vars = Vec::with_capacity(num_buttons);
    for _ in 0..num_buttons {
        let v = vars.add(variable().integer().min(0.0));
        button_vars.push(v);
    }

    let mut objective: Expression = 0.0.into();
    for &v in &button_vars {
        objective = objective + v;
    }

    let mut model = vars.minimise(objective).using(microlp);

    for (i, &t) in machine.target.iter().enumerate() {
        let mut expr: Expression = 0.0.into();
        for (j, button) in machine.buttons.iter().enumerate() {
            if button.contains(&i) {
                expr = expr + button_vars[j];
            }
        }
        model = model.with(expr.eq(t as f64));
    }

    let solution = model
        .solve()
        .expect("No ILP solution found for machine (part 2)");

    let mut presses_sum: i64 = 0;
    for v in button_vars {
        let val = solution.value(v);
        presses_sum += val.round() as i64;
    }

    presses_sum
}

fn part_two(input: &str) -> i64 {
    let machines = parse_input(input);
    machines.iter().map(min_presses_jolts).sum()
}
