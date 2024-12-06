use std::fs;
use std::collections::HashSet;

type RawGrid = Vec<Vec<char>>;
type Point = (isize, isize);

#[derive(PartialEq)]
enum Route {
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

    fn id(&self, p: &Point, dim: u32) -> isize {
        ((p.0 * self.ylen) + p.1).pow(dim)
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

        panic!("No guard found")
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
    let (route, _) = obstacle(grid, None);
    route.len()
}

#[test]
fn test_unique_steps() {
    let grid = parse("1");
    assert_eq!(unique_steps(&grid), 41)
}

fn valid_obstacle_count(grid: &Grid) -> usize {
    (0..grid.ylen).map(|y| {
        (0..grid.xlen)
            .filter(|x| grid.get(&(y, *x)) == '.')
            .filter(|x| {
                let (_, t) = obstacle(&grid, Some((y, *x)));
                t == Route::ClosedLoop
            })
            .count()
    }).sum()
}

fn obstacle(grid: &Grid, obstacle: Option<Point>) -> (HashSet<isize>, Route) {
    let mut guard_point = grid.guard();
    let mut route = HashSet::new();
    let mut direction = 0;
    let mut weight = 0;
    let obst = obstacle.unwrap_or((-1, -1));

    if obstacle.is_some() {
        weight = 1;
    }

    loop {
        let mut moved_point = guard_point;

        match direction {
            0 => moved_point.0 -= 1,
            1 => moved_point.1 += 1,
            2 => moved_point.0 += 1,
            3 => moved_point.1 -= 1,
            _ => panic!("Invalid direction")
        }

        let id = grid.id(&moved_point, (direction * weight) + 1);

        if weight > 0 && route.contains(&id) {
            return (route, Route::ClosedLoop)
        }

        if grid.out_of_bounds(&moved_point) {
            return (route, Route::OutOfBounds)
        }

        if grid.get(&moved_point) == '#' || moved_point == obst {
            direction += 1;
            direction %= 4;
        } else {
            guard_point = moved_point;
            route.insert(id);
        }
    }
}

#[test]
fn test_valid_obstacle_count() {
    let grid = parse("1");
    assert_eq!(valid_obstacle_count(&grid), 6)
}
