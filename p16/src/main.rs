use std::cmp::Ordering;
use std::fs;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [Point; 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

const COSTS: [(usize, usize); 3] = [
    (1, 1001),
    (0, 1),
    (3, 1001)
];

type RawGrid = Vec<Vec<char>>;
type Point = (isize, isize);

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

    fn id(&self, p: &Point) -> isize {
        (p.0 * self.ylen) + p.1
    }

    fn get_by_id(&self, id: &isize) -> char {
        let l = self.ylen;
        let y = id / l;
        let x = id % l;

        self.vector[y as usize][x as usize]
    }

    fn get(&self, p: &Point) -> char {
        self.vector[p.0 as usize][p.1 as usize]
    }

    fn lookup(&self, search: char) -> Point {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                if self.get(&(y, x)) == search {
                    return (y, x)
                }
            }
        }

        panic!("No point found")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StateWithPath {
    cost: usize,
    dir: usize,
    path: Vec<isize>
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
    let (cheapest, paths) = multi_route(&grid);
    println!("p1 {}", cheapest);
    println!("p2 {}", paths);
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn multi_route(grid: &Grid) -> (usize, usize) {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let end_id = grid.id(&end);
    let costs = HashMap::from(COSTS);

    let mut cheap = usize::MAX;
    let mut set: HashSet<isize> = HashSet::new();
    let mut cache = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(
        StateWithPath { cost: 0, dir: 1, path: vec![grid.id(&start)] }
    );

    while let Some(StateWithPath { cost, dir, path }) = heap.pop() {
        let id = path[path.len() - 1];
        let cache_key = id.pow((dir + 1) as u32);

        if id == end_id {
            if cost < cheap {
                cheap = cost;
                set.extend(&path);
            } else if cost == cheap {
                set.extend(&path);
            }
        }

        cache.insert(cache_key, cost);

        for (d, added_cost) in &costs {
            let new_dir = (dir + d) % DIRECTIONS.len();
            let t = DIRECTIONS[new_dir];
            let next_id = id + grid.id(&t);

            if grid.get_by_id(&next_id) == '#' {
                continue
            }

            let mut new_path = path.clone();
            new_path.push(next_id);

            let next_cost = cost + added_cost;
            let cache_key = next_id.pow((new_dir + 1) as u32);
            let hit = cache.get(&cache_key).unwrap_or(&usize::MAX);

            if &next_cost < hit {
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

    (cheap, set.len())
}

#[test]
fn test_multiple_routes() {
    let maze = parse("1");
    let (cost, r) = multi_route(&maze);
    assert_eq!(cost, 7036);
    assert_eq!(r, 45);

    let maze = parse("2");
    let (cost, r) = multi_route(&maze);
    assert_eq!(cost, 11048);
    assert_eq!(r, 64);
}
