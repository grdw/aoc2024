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

    fn to_point(&self, id: usize) -> Point {
        let y = id / self.size;
        let x = id % self.size;

        (y as i16, x as i16)
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

    println!("p1 {}", cheat_count_revised(&grid, 2, 100));
    println!("p2 {}", cheat_count_revised(&grid, 20, 100));
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

fn reconstruct_path(map: &HashMap<usize, usize>, id: usize) -> Vec<isize> {
    let mut route = vec![];
    let mut search = id;
    loop {
        let prev_id = map.get(&search);
        match prev_id {
            Some(x) => {
                route.insert(0, *x as isize);
                search = *x;
            },
            None => break
        }
    }
    route
}

fn path_len(map: &HashMap<usize, usize>, id: usize) -> usize {
    let mut route = 0;
    let mut search = id;
    loop {
        let prev_id = map.get(&search);
        match prev_id {
            Some(x) => {
                route += 1;
                search = *x;
            },
            None => break
        }
    }
    route
}

// Basic A* implementation
fn route(grid: &Grid) -> Option<Vec<isize>> {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score = vec![usize::MAX; grid.size * grid.size];

    heap.push(Node {
        position: start,
        cost: 0,
        estimate: manhattan_dist(&start, &end),
    });

    g_score[grid.id(&start)] = 0;

    while let Some(node) = heap.pop() {
        let id = grid.id(&node.position);
        if node.position == end {
            return Some(reconstruct_path(&came_from, id));
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
                came_from.insert(next_id, id);
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

// Basic A* implementation
fn route_with_cheat(
    grid: &Grid,
    start: &Point,
    end: &Point,
    length: usize,
    max: usize) -> Option<usize> {

    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score = vec![usize::MAX; grid.size * grid.size];

    heap.push(Node {
        position: *start,
        cost: length,
        estimate: manhattan_dist(&start, &end),
    });

    g_score[grid.id(&start)] = 0;

    while let Some(node) = heap.pop() {
        let id = grid.id(&node.position);

        if node.cost > max {
            return Some(max);
        }

        if &node.position == end {
            return Some(path_len(&came_from, id));
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
                came_from.insert(next_id, id);
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

fn cheat_count_revised(grid: &Grid, max: usize, seconds: usize) -> usize {
    let goal = grid.lookup('E');
    let regular = route(grid).unwrap();
    let t_no_cheating = regular.len();

    let mut count = 0;
    let mut cheat_list = vec![];

    for i in (0..regular.len()).rev() {
        let id = regular[i] as usize;
        let start = grid.to_point(id);

        for ny in 0..grid.size {
            for nx in 0..grid.size {
                let cheat_end = (ny as i16, nx as i16);
                if grid.is_wall(&cheat_end) {
                    continue
                }

                let m = manhattan_dist(&start, &cheat_end);
                if m > max {
                    continue
                }

                let total = i + m;
                if total >= (t_no_cheating - seconds + 1) {
                    continue
                }
                cheat_list.push((total, cheat_end));
            }
        }
    }

    let mut n = 0;
    let q = cheat_list.len();

    for (i, cheat_end) in cheat_list.into_iter() {
        if n % 1000 == 0 {
            println!("{} / {}", n, q);
        }

        let route_to_end = route_with_cheat(
            grid,
            &cheat_end,
            &goal,
            i,
            t_no_cheating - seconds + 1
        ).unwrap();

        let total = i + route_to_end;

        if total < t_no_cheating {
            let diff = t_no_cheating - total;

            if diff >= seconds {
                count += 1
            }
        }

        n += 1;
    }

    count
}

#[test]
fn test_cheat_count_no_revised() {
    let grid = parse("1");
    assert_eq!(cheat_count_revised(&grid, 20, 50), 285);
    assert_eq!(cheat_count_revised(&grid, 2, 12), 8);
}
