use std::cmp::Ordering;
use std::fs;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [(usize, (isize, isize)); 4] = [
    (90,  (0, 1)),
    (180, (1, 0)),
    (270, (0, -1)),
    (0,   (-1, 0))
];

const COSTS: [Upoint; 3] = [
    (90, 1001),
    (0, 1),
    (270, 1001)
];

type RawGrid = Vec<Vec<char>>;
type Point = (isize, isize);
type PointDir = (isize, isize, usize);
type Upoint = (usize, usize);

struct Grid {
    vector: RawGrid,
    ylen: isize,
    xlen: isize
}

impl Grid {
    fn new(vector: RawGrid) -> Grid {
        let ylen = vector.len() as isize;
        let xlen = vector[0].len() as isize;

        Grid {vector, ylen, xlen}
    }

    fn id(&self, p: &Point) -> usize {
        ((p.0 * self.ylen) + p.1) as usize
    }

    fn get(&self, p: &Point) -> char {
        self.vector[p.0 as usize][p.1 as usize]
    }

    fn lookup(&self, search: char) -> Point {
        for y in 0..self.ylen {
            for x in 0..self.ylen {
                if self.get(&(y, x)) == search {
                    return (y, x)
                }
            }
        }

        panic!("No point found")
    }

    #[allow(dead_code)]
    fn debug(&self, routes: &Vec<PointDir>) {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                let s = routes
                    .iter()
                    .find(|&&(py, px, _)| py == y && px == x);

                if s.is_some() {
                    print!("O");
                } else {
                    print!("{}", self.vector[y as usize][x as usize]);
                }
            }
            println!("");
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    dir: usize,
    point: Point
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StateWithPath {
    cost: usize,
    dir: usize,
    path: Vec<Point>
}

impl Ord for StateWithPath {
    fn cmp(&self, other: &StateWithPath) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for StateWithPath {
    fn partial_cmp(&self, other: &StateWithPath) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let grid = parse("input");
    let n = cheapest_route(&grid);
    println!("p1 {}", n);
    println!("p2 {}", multi_route(&grid, n));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn cheapest_route(grid: &Grid) -> usize {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let mut dist = vec![usize::MAX; (grid.ylen * grid.xlen) as usize];
    let directions = HashMap::from(DIRECTIONS);
    let costs = HashMap::from(COSTS);

    let mut heap = BinaryHeap::new();
    dist[grid.id(&start)] = 0;

    heap.push(State { cost: 0, dir: 90, point: start });

    while let Some(State { cost, dir, point }) = heap.pop() {
        let id = grid.id(&point);

        if point == end { return cost }
        if cost > dist[id] { continue }

        for (d, added_cost) in &costs {
            let new_dir = (dir + d) % 360;
            let (ty, tx) = directions[&new_dir];
            let (dy, dx) = (point.0 + ty, point.1 + tx);
            let next_id = grid.id(&(dy, dx));

            if grid.get(&(dy, dx)) == '#' {
                continue
            }

            let next = State {
                cost: cost + added_cost,
                dir: new_dir,
                point: (dy, dx)
            };

            if next.cost < dist[next_id] {
                dist[next_id] = next.cost;

                heap.push(next);
            }
        }
    }

    0
}

#[test]
fn test_cheapest_route() {
    let maze = parse("1");
    let cost = cheapest_route(&maze);
    assert_eq!(cost, 7036);

    let maze = parse("2");
    let cost = cheapest_route(&maze);
    assert_eq!(cost, 11048);
}

fn multi_route(grid: &Grid, max: usize) -> usize {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let directions = HashMap::from(DIRECTIONS);
    let costs = HashMap::from(COSTS);

    let mut set: HashSet<Point> = HashSet::new();
    let mut cache = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(
        StateWithPath { cost: 0, dir: 90, path: vec![start] }
    );

    while let Some(StateWithPath { cost, dir, path }) = heap.pop() {
        let point = path[path.len() - 1];

        if point == end && cost == max {
            set.extend(&path);
        }

        cache.insert((point, dir), cost);

        for (d, added_cost) in &costs {
            let new_dir = (dir + d) % 360;
            let (ty, tx) = directions[&new_dir];
            let (dy, dx) = (point.0 + ty, point.1 + tx);

            if grid.get(&(dy, dx)) == '#' {
                continue
            }

            let mut new_path = path.clone();
            new_path.push((dy, dx));

            let next_cost = cost + added_cost;
            let hit = cache.get(&((dy, dx), new_dir));

            if hit.is_none_or(|&s| s > next_cost) {
                heap.push(
                    StateWithPath {
                        cost: next_cost,
                        dir: new_dir,
                        path: new_path
                    }
                );
            }
        }
    }
    set.len()
}

#[test]
fn test_multiple_routes() {
    let maze = parse("1");
    let r = multi_route(&maze, 7036);
    assert_eq!(r, 45);

    let maze = parse("2");
    let r = multi_route(&maze, 11048);
    assert_eq!(r, 64);
}
