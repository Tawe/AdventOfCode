const INPUT: &str = include_str!("./input.txt");

type Point = [i64; 3];

fn main() {
    let part_one_result: i64 = part_one(INPUT, 1000);
    println!("Part One: {part_one_result}");

    let part_two_result: i64 = part_two(INPUT);
    println!("Part Two: {part_two_result}");
}
fn part_one(input: &str, k_pairs: usize) -> i64 {
    let points = parse_points(input);
    let n = points.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d2 = dist2(points[i], points[j]);
            edges.push((d2, i, j));
        }
    }
    edges.sort_by_key(|e| e.0);

    let mut dsu = DisjointSet::new(n);

    let limit = k_pairs.min(edges.len());
    for i in 0..limit {
        let (_d2, a, b) = edges[i];
        dsu.union(a, b);
    }

    use std::collections::HashMap;
    let mut comp_sizes: HashMap<usize, i64> = HashMap::new();
    for i in 0..n {
        let root = dsu.find(i);
        *comp_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<i64> = comp_sizes.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    if sizes.len() < 3 {
        panic!("Expected at least 3 circuits.");
    }

    sizes[0] * sizes[1] * sizes[2]
}


fn part_two(input: &str) -> i64 {
    let points = parse_points(input);
    let n = points.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let d2 = dist2(points[i], points[j]);
            edges.push((d2, i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut dsu = DisjointSet::new(n);
    let mut components = n;

    for (_d2, a, b) in edges {
        let ra = dsu.find(a);
        let rb = dsu.find(b);

        if ra != rb {
            dsu.union(a, b);
            components -= 1;

            if components == 1 {
                let xa = points[a][0];
                let xb = points[b][0];
                return xa * xb;
            }
        }
    }

    panic!("Never reached a single connected component.");
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let parts: Vec<i64> =
                line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            assert!(parts.len() == 3);
            [parts[0], parts[1], parts[2]]
        })
        .collect()
}

fn dist2(a: Point, b: Point) -> i64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}
