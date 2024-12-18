use std::cmp::Ordering;
use std::fs;
use std::collections::{BinaryHeap, HashMap};

const DIRECTIONS: [Point; 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

type Point = (i16, i16);

struct Grid {
    vector: Vec<Vec<char>>,
    size: usize
}

impl Grid {
    fn new(size: usize, corruptions: &[Point]) -> Grid {
        let mut vector = vec![vec!['.'; size]; size];
        for (cy, cx) in corruptions {
            vector[*cy as usize][*cx as usize] = '#';
        }
        Grid { size, vector }
    }

    fn is_corrupted(&self, p: &Point) -> bool {
        self.vector[p.0 as usize][p.1 as usize] == '#'
    }

    fn out_of_bounds(&self, p: &Point) -> bool {
        let s = self.size as i16;

        p.0 < 0 || p.1 < 0 || p.0 >= s || p.1 >= s
    }

    fn id(&self, p: &Point) -> usize {
        let y = p.0 as usize;
        let x = p.1 as usize;

        (y * self.size) + x
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", self.vector[y as usize][x as usize]);
            }
            println!("");
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: Point,
    cost: usize, // g-score: cost from start to this node
    estimate: usize, // f-score: g-score + heuristic estimate to goal
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate) // Reverse ordering for min-heap behavior
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let points = parse("input");

    println!("p1 {}", route(70, &points[0..1024]));
    //println!("p2 {}", paths);
}

fn parse(input: &'static str) -> Vec<Point> {
    fs::read_to_string(input).unwrap().lines().map(|line| {
        let (left, right) = line.split_once(",").unwrap();

        (right.parse::<i16>().unwrap(), left.parse::<i16>().unwrap())
    }).collect()
}

fn heuristic(s: &Point, e: &Point) -> usize {
    let dx = if s.0 > e.0 { s.0 - e.0 } else { e.0 - s.0 };
    let dy = if s.1 > e.1 { s.1 - e.1 } else { e.1 - s.1 };
    (dx + dy) as usize
}

fn route(size: i16, points: &[Point]) -> usize {
    let start: Point = (0, 0);
    let end: Point = (size, size);
    let size_u = (size + 1) as usize;
    let grid = Grid::new(size_u, points);
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        position: start,
        cost: 0,
        estimate: heuristic(&start, &end)
	});

    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score = HashMap::new();
    for y in 0..grid.size {
        for x in 0..grid.size {
            let p = (y as i16, x as i16);
            g_score.insert(grid.id(&p), usize::MAX);
        }
    }
    g_score.insert(grid.id(&start), 0);

    while let Some(Node { position, cost: _, estimate: _ }) = heap.pop() {
        let id = grid.id(&position);
        if position == end {
            return reconstruct_path(&came_from, id);
        }

        for (ty, tx) in &DIRECTIONS {
            let np = (position.0 + ty, position.1 + tx);

            if grid.out_of_bounds(&np) || grid.is_corrupted(&np)  {
                continue
            }

            let next_id = grid.id(&np);
            let new_score = g_score[&id] + 1;

            if new_score < g_score[&next_id] {
                came_from.insert(next_id, id);
                g_score.insert(next_id, new_score);
                heap.push(Node {
                    position: np,
                    cost: new_score,
                    estimate: new_score + heuristic(&np, &end)
                });
            }
        }
    }

    0
}

fn reconstruct_path(map: &HashMap<usize, usize>, current: usize) -> usize {
    let mut count = 0;
    let mut search = current;
    loop {
        let prev_id = map.get(&search);
        match prev_id {
            Some(x) => {
                search = *x;
                count += 1;
            },
            None => break
        }
    }
    count
}

#[test]
fn test_multiple_routes() {
    let points = parse("1");
    assert_eq!(route(6, &points[0..12]), 22);
}
