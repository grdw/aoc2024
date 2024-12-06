use std::fs;
use std::collections::HashSet;

type RawGrid = Vec<Vec<char>>;
type Point = (isize, isize);

#[derive(PartialEq)]
enum GuardRoute {
    OutOfBounds,
    ClosedLoop,
}

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

    fn out_of_bounds(&self, p: &Point) -> bool {
        p.0 < 0 || p.1 < 0 || p.0 >= self.ylen || p.1 >= self.xlen
    }

    fn get(&self, p: &Point) -> char {
        self.vector[p.0 as usize][p.1 as usize]
    }

    fn guard(&self) -> Point {
        for y in 0..self.ylen {
            for x in 0..self.ylen {
                if self.get(&(y, x)) == '^' {
                    return (y, x)
                }
            }
        }
        (0, 0)
    }
}

fn main() {
    let grid = parse("input");
    println!("p1 {}", unique_steps(&grid));
    println!("p2 {}", valid_obstacle_count(&grid));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn unique_steps(grid: &Grid) -> usize {
    let mut guard_point = grid.guard();
    let mut direction = 0;
    let mut route = HashSet::new();

    loop {
        let mut moved_point = guard_point;

        match direction {
            0 => moved_point.0 -= 1,
            1 => moved_point.1 += 1,
            2 => moved_point.0 += 1,
            3 => moved_point.1 -= 1,
            _ => panic!("Invalid direction")
        }

        if grid.out_of_bounds(&moved_point) {
            break;
        }

        if grid.get(&moved_point) == '#' {
            direction += 1;
            direction %= 4;
        } else {
            guard_point = moved_point;
            route.insert(guard_point);
        }
    }

    route.len()
}

#[test]
fn test_unique_steps() {
    let grid = parse("1");
    assert_eq!(unique_steps(&grid), 41)
}

fn valid_obstacle_count(grid: &Grid) -> usize {
    let mut obstacle_count = 0;
    for y in 0..grid.ylen {
        for x in 0..grid.xlen {
            if grid.get(&(y, x)) == '.' {
                let t = try_obstacle(&grid, &(y, x));
                if t == GuardRoute::ClosedLoop {
                    obstacle_count += 1
                }
            }
        }
    }
    obstacle_count
}

fn try_obstacle(grid: &Grid, obstacle: &Point) -> GuardRoute {
    let mut guard_point = grid.guard();
    let mut direction = 0;
    let mut route = HashSet::new();

    loop {
        let mut moved_point = guard_point;

        match direction {
            0 => moved_point.0 -= 1,
            1 => moved_point.1 += 1,
            2 => moved_point.0 += 1,
            3 => moved_point.1 -= 1,
            _ => panic!("Invalid direction")
        }

        let n = route.get(&(moved_point, direction));
        if n.is_some() {
            return GuardRoute::ClosedLoop
        }

        if grid.out_of_bounds(&moved_point) {
            return GuardRoute::OutOfBounds
        }

        if grid.get(&moved_point) == '#' || &moved_point == obstacle {
            direction += 1;
            direction %= 4;
        } else {
            guard_point = moved_point;
            route.insert((guard_point, direction));
        }
    }
}

#[test]
fn test_valid_obstacle_count() {
    let grid = parse("1");
    assert_eq!(valid_obstacle_count(&grid), 6)
}
