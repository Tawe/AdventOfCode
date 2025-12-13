use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let part_one_result: i64 = part_one(INPUT);
    println!("Part One: {part_one_result}");
    println!("Part Two: Merry Christmas");
}

fn part_one(input: &str) -> i64 {
    let (shapes, regions) = parse_input(input);

    let mut count = 0i64;

    for region in regions {
        if can_fit_all(&shapes, &region) {
            count += 1;
        }
    }

    count
}



#[derive(Debug, Clone)]
struct ShapeOrientation {
    cells: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

#[derive(Debug, Clone)]
struct Shape {
    id: usize,
    orientations: Vec<ShapeOrientation>,
    area: usize,
}

#[derive(Debug, Clone)]
struct Region {
    width: i32,
    height: i32,
    counts: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut shape_blocks: Vec<String> = Vec::new();
    let mut region_lines: Vec<String> = Vec::new();

    let mut current_shape_block: Vec<String> = Vec::new();
    let mut in_regions = false;

    for raw_line in input.lines() {
        let line = raw_line.trim_end();

        if line.is_empty() {
            if !in_regions && !current_shape_block.is_empty() {
                shape_blocks.push(current_shape_block.join("\n"));
                current_shape_block.clear();
            }
            continue;
        }

        if !in_regions && is_region_line(line) {
           in_regions = true;
            if !current_shape_block.is_empty() {
                shape_blocks.push(current_shape_block.join("\n"));
                current_shape_block.clear();
            }
            region_lines.push(line.to_string());
        } else if in_regions {
            region_lines.push(line.to_string());
        } else {
            current_shape_block.push(line.to_string());
        }
    }

    if !in_regions && !current_shape_block.is_empty() {
        shape_blocks.push(current_shape_block.join("\n"));
    }

    let shapes = parse_shapes(&shape_blocks);
    let regions = parse_regions(&region_lines, shapes.len());

    (shapes, regions)
}

fn is_region_line(line: &str) -> bool {
    if let Some((wh, _rest)) = line.split_once(':') {
        let mut parts = wh.split('x');
        if let (Some(w), Some(h)) = (parts.next(), parts.next()) {
            return w.chars().all(|c| c.is_ascii_digit())
                && h.chars().all(|c| c.is_ascii_digit());
        }
    }
    false
}

fn parse_shapes(blocks: &[String]) -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::new();
    for block in blocks {
        let mut lines = block.lines();
        let header = lines
            .next()
            .expect("Shape block must start with an index line like `0:`")
            .trim();
        let id_str = header.trim_end_matches(':').trim();
        let id: usize = id_str
            .parse()
            .expect("Shape index header should be a number followed by ':'");

        let mut raw_cells: Vec<(i32, i32)> = Vec::new();
        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    raw_cells.push((x as i32, y as i32));
                }
            }
        }

        let area = raw_cells.len();
        let orientations = generate_orientations(&raw_cells);

        shapes.push(Shape {
            id,
            orientations,
            area,
        });
    }

    shapes.sort_by_key(|s| s.id);
    shapes
}

fn parse_regions(lines: &[String], shape_count: usize) -> Vec<Region> {
    let mut regions = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (wh, rest) = line
            .split_once(':')
            .expect("Region line must look like `WxH: counts...`");

        let mut wh_it = wh.split('x');
        let width: i32 = wh_it
            .next()
            .expect("Width missing")
            .trim()
            .parse()
            .expect("Width must be an integer");
        let height: i32 = wh_it
            .next()
            .expect("Height missing")
            .trim()
            .parse()
            .expect("Height must be an integer");

        let mut counts: Vec<usize> = rest
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Region counts must be integers"))
            .collect();

        if counts.len() < shape_count {
            counts.resize(shape_count, 0);
        } else if counts.len() > shape_count {
            counts.truncate(shape_count);
        }

        regions.push(Region {
            width,
            height,
            counts,
        });
    }

    regions
}

fn generate_orientations(cells: &[(i32, i32)]) -> Vec<ShapeOrientation> {
    let mut result = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for flip in 0..2 {
        let flipped: Vec<(i32, i32)> = if flip == 0 {
            cells.to_vec()
        } else {
            cells.iter().map(|&(x, y)| (-x, y)).collect()
        };

        for rot in 0..4 {
            let rotated: Vec<(i32, i32)> = flipped
                .iter()
                .map(|&(x, y)| match rot {
                    0 => (x, y),
                    1 => (-y, x),
                    2 => (-x, -y),
                    3 => (y, -x),
                    _ => unreachable!(),
                })
                .collect();

            let min_x = rotated.iter().map(|(x, _)| *x).min().unwrap();
            let min_y = rotated.iter().map(|(_, y)| *y).min().unwrap();

            let mut norm: Vec<(i32, i32)> = rotated
                .into_iter()
                .map(|(x, y)| (x - min_x, y - min_y))
                .collect();
            norm.sort();

            let key = norm
                .iter()
                .map(|(x, y)| format!("{x},{y}"))
                .collect::<Vec<_>>()
                .join(";");
            if !seen.insert(key) {
                continue;
            }

            let max_x = norm.iter().map(|(x, _)| *x).max().unwrap();
            let max_y = norm.iter().map(|(_, y)| *y).max().unwrap();

            result.push(ShapeOrientation {
                cells: norm,
                width: max_x + 1,
                height: max_y + 1,
            });
        }
    }

    result
}

fn can_fit_all(shapes: &[Shape], region: &Region) -> bool {
    let board_area = (region.width * region.height) as usize;

    let total_area: usize = shapes
        .iter()
        .enumerate()
        .map(|(i, s)| s.area * region.counts.get(i).copied().unwrap_or(0))
        .sum();

    if total_area > board_area {
        return false;
    }

    let placements_by_shape = compute_placements(shapes, region);
    for (i, count) in region.counts.iter().enumerate() {
        if *count > 0 && placements_by_shape[i].is_empty() {
            return false;
        }
    }

    let mut instances: Vec<usize> = Vec::new();
    for (shape_idx, &count) in region.counts.iter().enumerate() {
        for _ in 0..count {
            instances.push(shape_idx);
        }
    }

    if instances.is_empty() {
        return true;
    }

    instances.sort_by_key(|&idx| std::cmp::Reverse(shapes[idx].area));

    let mut board = vec![false; board_area];
    backtrack_place(
        0,
        &instances,
        &placements_by_shape,
        &mut board,
    )
}

fn compute_placements(shapes: &[Shape], region: &Region) -> Vec<Vec<Vec<usize>>> {
    let mut result: Vec<Vec<Vec<usize>>> = vec![Vec::new(); shapes.len()];

    let width = region.width;
    let height = region.height;

    for shape in shapes {
        let mut shape_placements: Vec<Vec<usize>> = Vec::new();

        for ori in &shape.orientations {
            if ori.width > width || ori.height > height {
                continue;
            }

            for y in 0..=(height - ori.height) {
                for x in 0..=(width - ori.width) {
                    let mut placement: Vec<usize> = Vec::with_capacity(ori.cells.len());
                    for &(cx, cy) in &ori.cells {
                        let xx = x + cx;
                        let yy = y + cy;
                        let idx = (yy * width + xx) as usize;
                        placement.push(idx);
                    }
                    shape_placements.push(placement);
                }
            }
        }

        result[shape.id] = shape_placements;
    }

    result
}

fn backtrack_place(
    instance_idx: usize,
    instances: &[usize],
    placements_by_shape: &[Vec<Vec<usize>>],
    board: &mut [bool],
) -> bool {
    if instance_idx == instances.len() {
        return true;
    }

    let shape_idx = instances[instance_idx];

    'placements: for placement in &placements_by_shape[shape_idx] {
        for &idx in placement {
            if board[idx] {
                continue 'placements;
            }
        }

        for &idx in placement {
            board[idx] = true;
        }

        if backtrack_place(instance_idx + 1, instances, placements_by_shape, board) {
            return true;
        }

        for &idx in placement {
            board[idx] = false;
        }
    }

    false
}
