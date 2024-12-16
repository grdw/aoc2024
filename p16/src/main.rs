use std::cmp::Ordering;
use std::fs;
use std::collections::{BinaryHeap, HashMap};

const DIRECTIONS: [(usize, (isize, isize)); 4] = [
    (90,  (0, 1)),
    (180, (1, 0)),
    (270, (0, -1)),
    (0,   (-1, 0))
];

const COSTS: [(usize, usize); 3] = [
    (90, 1001),
    (0, 1),
    (270, 1001)
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
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    point: Point,
    dir: usize,
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

fn main() {
    let grid = parse("input");
    println!("p1 {}", cheapest_route(&grid).unwrap_or(0));
    //println!("p2 {}", valid_obstacle_count(&grid));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn cheapest_route(grid: &Grid) -> Option<usize> {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let mut dist = vec![usize::MAX; (grid.ylen * grid.xlen) as usize];
    let directions = HashMap::from(DIRECTIONS);
    let costs = HashMap::from(COSTS);

    let mut heap = BinaryHeap::new();
    dist[grid.id(&start)] = 0;

    heap.push(State { cost: 0, point: start, dir: 90 });

    while let Some(State { cost, point, dir }) = heap.pop() {
        let id = grid.id(&point);
        if point == end { return Some(cost) }
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
                point: (dy, dx),
                dir: new_dir,
            };

            if next.cost < dist[next_id] {
                heap.push(next);
                dist[next_id] = next.cost;
            }
        }
    }

    None
}

#[test]
fn test_cheapest_route() {
    let maze = parse("1");
    assert_eq!(cheapest_route(&maze), Some(7036));

    let maze = parse("2");
    assert_eq!(cheapest_route(&maze), Some(11048));
}
