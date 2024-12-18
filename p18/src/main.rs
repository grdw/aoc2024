use std::cmp::Ordering;
use std::fs;
use std::collections::{BinaryHeap, HashMap};

const SIZE: i16 = 70;
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

    println!("p1 {}", route(SIZE, &points[0..1024]).unwrap());
    println!("p2 {}", last_point(SIZE, &points));
}

fn parse(input: &'static str) -> Vec<Point> {
    fs::read_to_string(input).unwrap().lines().map(|line| {
        let (left, right) = line.split_once(",").unwrap();

        (right.parse::<i16>().unwrap(), left.parse::<i16>().unwrap())
    }).collect()
}

// This is the Manhattan distance
fn heuristic(s: &Point, e: &Point) -> usize {
    let dx = e.0 - s.0;
    let dy = e.1 - s.1;
    (dx + dy) as usize
}

fn reconstruct_path(map: &HashMap<usize, usize>, id: usize) -> usize {
    let mut count = 0;
    let mut search = id;
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

// Basic A* implementation
fn route(size: i16, points: &[Point]) -> Option<usize> {
    let start: Point = (0, 0);
    let end: Point = (size, size);
    let size_u = (size + 1) as usize;
    let grid = Grid::new(size_u, points);

    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score = vec![usize::MAX; size_u * size_u];

    heap.push(Node {
        position: start,
        cost: 0,
        estimate: heuristic(&start, &end)
    });

    g_score[0] = 0;

    while let Some(node) = heap.pop() {
        let id = grid.id(&node.position);
        if node.position == end {
            return Some(reconstruct_path(&came_from, id));
        }

        for (ty, tx) in &DIRECTIONS {
            let np = (node.position.0 + ty, node.position.1 + tx);

            if grid.out_of_bounds(&np) || grid.is_corrupted(&np)  {
                continue
            }

            let next_id = grid.id(&np);
            let new_score = g_score[id] + 1;

            if new_score < g_score[next_id] {
                came_from.insert(next_id, id);
                g_score[next_id] = new_score;
                heap.push(Node {
                    position: np,
                    cost: new_score,
                    estimate: new_score + heuristic(&np, &end)
                });
            }
        }
    }

    None
}

#[test]
fn test_multiple_routes() {
    let points = parse("1");
    assert_eq!(route(6, &points[0..12]), Some(22));
}

fn last_point(size: i16, points: &[Point]) -> String {
    for i in 0..points.len() {
        if route(size, &points[0..i]).is_none() {
            let (y, x) = points[i - 1];

            return format!("{},{}", x, y);
        }
    }

    String::new()
}

#[test]
fn test_last_point() {
    let points = parse("1");
    assert_eq!(last_point(6, &points), String::from("6,1"));
}
