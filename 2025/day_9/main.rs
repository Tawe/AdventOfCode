use std::cmp::{min, max};
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}

fn parse_points(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x: i32 = parts.next().unwrap().trim().parse().unwrap();
            let y: i32 = parts.next().unwrap().trim().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn part_one(input: &str) -> i64 {
    let points = parse_points(input);

    let mut max_area: i64 = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let width = (x1 - x2).abs() as i64 + 1;
            let height = (y1 - y2).abs() as i64 + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn part_two(input: &str) -> i64 {
    let points = parse_points(input);
    if points.len() < 2 {
        return 0;
    }
    let mut xs: Vec<i32> = points.iter().map(|(x, _)| *x).collect();
    xs.sort();
    xs.dedup();

    let mut ys: Vec<i32> = points.iter().map(|(_, y)| *y).collect();
    ys.sort();
    ys.dedup();

    let nx = xs.len();
    let ny = ys.len();

    let mut x_index = HashMap::new();
    for (i, &x) in xs.iter().enumerate() {
        x_index.insert(x, i);
    }

    let mut y_index = HashMap::new();
    for (i, &y) in ys.iter().enumerate() {
        y_index.insert(y, i);
    }

    let w = nx * 2 - 1;
    let h = ny * 2 - 1;

    let mut col_width = vec![0_i64; w];
    for idx in 0..w {
        if idx % 2 == 0 {
            col_width[idx] = 1;
        } else {
            let k = idx / 2;
            let gap = xs[k + 1] - xs[k] - 1;
            col_width[idx] = if gap > 0 { gap as i64 } else { 0 };
        }
    }

    let mut row_height = vec![0_i64; h];
    for idx in 0..h {
        if idx % 2 == 0 {
            row_height[idx] = 1;
        } else {
            let k = idx / 2;
            let gap = ys[k + 1] - ys[k] - 1;
            row_height[idx] = if gap > 0 { gap as i64 } else { 0 };
        }
    }

    let w_ext = w + 2;
    let h_ext = h + 2;
    let mut wall = vec![vec![false; w_ext]; h_ext];

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];

        if x1 == x2 {
            let kx = x_index[&x1];
            let c_ext = 2 * kx + 1; 

            let yi = y_index[&y1];
            let yj = y_index[&y2];
            let lo = min(yi, yj);
            let hi = max(yi, yj);

            for k in (2 * lo)..=(2 * hi) {
                let r_ext = k + 1; 
                wall[r_ext][c_ext] = true;
            }
        } else if y1 == y2 {
            let ky = y_index[&y1];
            let r_ext = 2 * ky + 1;

            let xi = x_index[&x1];
            let xj = x_index[&x2];
            let lo = min(xi, xj);
            let hi = max(xi, xj);

            for k in (2 * lo)..=(2 * hi) {
                let c_ext = k + 1; 
                wall[r_ext][c_ext] = true;
            }
        } else {
            panic!(
                "Non axis-aligned edge between ({},{}) and ({},{})",
                x1, y1, x2, y2
            );
        }
    }

    let mut visited = vec![vec![false; w_ext]; h_ext];
    let mut queue = VecDeque::new();

    if !wall[0][0] {
        visited[0][0] = true;
        queue.push_back((0_usize, 0_usize));
    }

    let dirs = [(1_i32, 0_i32), (-1, 0), (0, 1), (0, -1)];

    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if nr >= h_ext || nc >= w_ext {
                continue;
            }
            if wall[nr][nc] || visited[nr][nc] {
                continue;
            }
            visited[nr][nc] = true;
            queue.push_back((nr, nc));
        }
    }

    let mut prefix = vec![vec![0_i64; w_ext]; h_ext];

    for r_ext in 1..=h {
        for c_ext in 1..=w {
            let mut cell_area = 0_i64;

            if wall[r_ext][c_ext] || !visited[r_ext][c_ext] {
                let r_idx = r_ext - 1;
                let c_idx = c_ext - 1;
                cell_area = row_height[r_idx] * col_width[c_idx];
            }

            prefix[r_ext][c_ext] = cell_area
                + prefix[r_ext - 1][c_ext]
                + prefix[r_ext][c_ext - 1]
                - prefix[r_ext - 1][c_ext - 1];
        }
    }
    
    let mut grid_points = Vec::with_capacity(points.len());
    for &(x, y) in &points {
        let yi = y_index[&y];
        let xi = x_index[&x];
        let r_ext = 2 * yi + 1;
        let c_ext = 2 * xi + 1;
        grid_points.push((r_ext, c_ext));
    }

    let mut max_area: i64 = 0;

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (r1, c1) = grid_points[i];

        for j in (i + 1)..points.len() {
            let (x2, y2) = points[j];
            let (r2, c2) = grid_points[j];

            let r_lo = min(r1, r2);
            let r_hi = max(r1, r2);
            let c_lo = min(c1, c2);
            let c_hi = max(c1, c2);
            
            let rect_area =
                ((x1 - x2).abs() as i64 + 1) * ((y1 - y2).abs() as i64 + 1);

            let sum_allowed = prefix[r_hi][c_hi]
                - prefix[r_lo - 1][c_hi]
                - prefix[r_hi][c_lo - 1]
                + prefix[r_lo - 1][c_lo - 1];

            if sum_allowed == rect_area && rect_area > max_area {
                max_area = rect_area;
            }
        }
    }

    max_area
}