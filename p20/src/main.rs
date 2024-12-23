use std::fs;
use std::collections::{HashMap};

const DIRECTIONS: [Point; 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

type Point = (i16, i16);
type Route = Vec<Point>;
type Cache = HashMap<Point, usize>;

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
}

fn main() {
    let grid = parse("input");
    let route = simple_route(&grid);

    println!("p1 {}", cheat_count(&grid, &route, 2, 100));
    println!("p2 {}", cheat_count(&grid, &route, 20, 100));
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

// Only useful if there's a single direction to walk in from 'start'
fn simple_route(grid: &Grid) -> Route {
    let start = grid.lookup('S');
    let end = grid.lookup('E');

    let mut route: Vec<Point> = vec![start];
    let mut step = start;

    while step != end {
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

fn cheat_count(grid: &Grid, route: &Route, c: usize, s: usize) -> usize {
    let t_no_cheating = route.len();

    let mut count = 0;
    let mut cache = HashMap::new();

    for i in (0..route.len()).rev() {
        let start = route[i];

        for ny in 0..grid.size {
            for nx in 0..grid.size {
                let cheat_end = (ny as i16, nx as i16);

                if grid.is_wall(&cheat_end) {
                    continue
                }

                let m = manhattan_dist(&start, &cheat_end);

                if m > c {
                    continue
                }

                let goal_len = goal_len(&route, &cheat_end, &mut cache);
                let subtotal = i + m + goal_len;

                if subtotal >= t_no_cheating {
                    continue
                }

                if (t_no_cheating - subtotal) >= s {
                    count += 1;
                }
            }
        }
    }

    count
}

fn goal_len(route: &Route, point: &Point, cache: &mut Cache) -> usize {
    match cache.get(point) {
        Some(goal_len) => *goal_len,
        None => {
            let q = route
                .iter()
                .position(|p| p == point)
                .unwrap();

            let goal_len = route.len() - q - 1;
            cache.insert(*point, goal_len);
            goal_len
        }
    }
}

#[test]
fn test_cheat_count_no_revised() {
    let grid = parse("1");
    let route = simple_route(&grid);
    assert_eq!(cheat_count(&grid, &route, 20, 50), 285);
    assert_eq!(cheat_count(&grid, &route, 2, 12), 8);
}
