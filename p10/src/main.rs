use std::fs;
use std::collections::{HashSet, VecDeque};

const TRANSLATIONS: [Point; 4] = [
    (-1, 0), // TOP CENTRE
    (0, -1), // CENTRE LEFT
    (0, 1),  // CENTRE RIGHT
    (1, 0),  // BOTTOM CENTRE
];

type RawGrid = Vec<Vec<u8>>;
type Point = (isize, isize);
type PointValue = (isize, isize, u8);

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

    fn out_of_bounds(&self, y: isize, x: isize) -> bool {
        y < 0 || x < 0 || y >= self.ylen || x >= self.xlen
    }

    fn get(&self, y: isize, x: isize) -> u8 {
        self.vector[y as usize][x as usize]
    }
}

fn main() {
    let grid = parse("input");
    println!("p1 {}", trailhead_scores(&grid));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().map(|i| i.to_digit(10).unwrap() as u8).collect()
    }).collect();

    Grid::new(vector)
}

fn trailhead_scores(grid: &Grid) -> usize {
    let mut starts = vec![];
    for y in 0..grid.ylen {
        for x in 0..grid.xlen {
            if grid.get(y, x) == 0 {
                starts.push((y, x, 0));
            }
        }
    }

    starts
        .iter()
        .map(|p| trailhead_score(grid, *p))
        .sum()
}

fn trailhead_score(grid: &Grid, start: PointValue) -> usize {
    let mut set = HashSet::new();
    let mut deq = VecDeque::new();
    deq.push_back(start);

    while let Some((y, x, v)) = deq.pop_front() {
        for (dy, dx) in &TRANSLATIONS {
            let (ty, tx) = (y - dy, x - dx);

            if !grid.out_of_bounds(ty, tx) {
                let vv = grid.get(ty, tx);
                if vv == v + 1 && vv == 9 {
                    set.insert((ty, tx));
                    continue
                }

                if vv == v + 1 {
                    deq.push_back((ty, tx, vv));
                }
            }
        }
    }

    set.len()
}

#[test]
fn test_trailhead_score() {
    let grid = parse("1");
    assert_eq!(trailhead_scores(&grid), 1);

    let grid = parse("3");
    assert_eq!(trailhead_scores(&grid), 2);

    let grid = parse("4");
    assert_eq!(trailhead_scores(&grid), 4);

    let grid = parse("2");
    assert_eq!(trailhead_scores(&grid), 36);
}
