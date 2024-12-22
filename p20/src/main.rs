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
    fn new(vector: Vec<Vec<char>>) -> Grid {
        let size = vector.len();
        Grid { size, vector }
    }

    fn is_wall(&self, p: &Point) -> bool {
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

    fn lookup(&self, search: char) -> Point {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.vector[y][x] == search {
                    return (y as i16, x as i16)
                }
            }
        }
        return (0, 0)
    }

    #[allow(dead_code)]
    fn debug(&self, points: &Vec<isize>, c: char) {
        for y in 0..self.size {
            for x in 0..self.size {
                let did = self.id(&(y as i16, x as i16)) as isize;

                if points.iter().find(|&&id| did == id).is_some() {
                    print!("{}", c);
                } else {
                    print!("{}", self.vector[y][x]);
                }
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
    let grid = parse("input");

    println!("p1 {}", cheat_count(&grid, 2, 100));
    println!("p2 {}", cheat_count(&grid, 20, 100));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Grid::new(vector)
}

// This is the Manhattan distance
fn manhattan_dist(s: &Point, e: &Point) -> usize {
    let dx = if s.0 < e.0 { e.0 - s.0 } else { s.0 - e.0 };
    let dy = if s.1 < e.1 { e.1 - s.1 } else { s.1 - e.1 };
    (dx + dy) as usize
}

fn simple_route(grid: &Grid, start: &Point, end: &Point) -> Vec<Point> {
    let mut route: Vec<Point> = vec![*start];
    let mut step = *start;

    while step != *end {
        let next = DIRECTIONS
            .iter()
            .map(|(ty, tx)| (step.0 + ty, step.1 + tx))
            .find(|n| !grid.is_wall(&n) && !route.contains(&n))
            .unwrap();

        route.push(next);
        step = next;
    }
    route
}

// Basic A* implementation
fn route(grid: &Grid, start: &Point, end: &Point) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut g_score = vec![usize::MAX; grid.size * grid.size];

    heap.push(Node {
        position: *start,
        cost: 0,
        estimate: manhattan_dist(&start, &end),
    });

    g_score[grid.id(&start)] = 0;

    while let Some(node) = heap.pop() {
        let id = grid.id(&node.position);

        if &node.position == end {
            return Some(g_score[id]);
        }

        for (ty, tx) in &DIRECTIONS {
            let np = (node.position.0 + ty, node.position.1 + tx);

            if grid.out_of_bounds(&np) {
                continue
            }

            let new_score = g_score[id] + 1;
            let next_id = grid.id(&np);
            if grid.is_wall(&np) {
                continue
            }

            if new_score < g_score[next_id] {
                g_score[next_id] = new_score;

                heap.push(Node {
                    position: np,
                    cost: new_score,
                    estimate: new_score + manhattan_dist(&np, &end),
                });
            }
        }
    }

    None
}

fn cheat_count(grid: &Grid, cheat_time: usize, seconds: usize) -> usize {
    let start = grid.lookup('S');
    let goal = grid.lookup('E');
    let regular = simple_route(grid, &start, &goal);
    let t_no_cheating = regular.len();
    let roof = t_no_cheating - seconds + 1;

    let mut count = 0;
    let mut cache = HashMap::new();

    for i in (0..regular.len()).rev() {
        let start = regular[i];

        for ny in 0..grid.size {
            for nx in 0..grid.size {
                let cheat_end = (ny as i16, nx as i16);

                if grid.is_wall(&cheat_end) {
                    continue
                }

                let m = manhattan_dist(&start, &cheat_end);

                if m > cheat_time {
                    continue
                }

                let mut total = i + m;

                let to_end = manhattan_dist(&cheat_end, &goal);

                if total + to_end >= roof {
                    continue
                }

                match cache.get(&cheat_end) {
                    Some(goal_len) => total += goal_len,
                    None => {
                        let goal_len = route(
                            grid,
                            &cheat_end,
                            &goal
                        ).unwrap();

                        cache.insert(cheat_end, goal_len);
                        total += goal_len;
                    }
                }

                if total >= t_no_cheating {
                    continue
                }

                let diff = t_no_cheating - total;
                if diff >= seconds {
                    count += 1;
                }
            }
        }
    }

    count
}

#[test]
fn test_cheat_count_no_revised() {
    let grid = parse("1");
    assert_eq!(cheat_count(&grid, 20, 50), 285);
    assert_eq!(cheat_count(&grid, 2, 12), 8);
}
