use std::fs;
use std::collections::{HashMap};

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

// Only useful if there's a single direction to walk in from 'start'
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
                        let q = regular
                            .iter()
                            .position(|&p| p == cheat_end)
                            .unwrap();

                        let goal_len = regular.len() - q - 1;
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
