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
    let (scores, ratings) = trailhead_totals(&grid);
    println!("p1 {}", scores);
    println!("p2 {}", ratings);
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().map(|i| i.to_digit(10).unwrap() as u8).collect()
    }).collect();

    Grid::new(vector)
}

fn trailhead_totals(grid: &Grid) -> (usize, usize) {
    let starts = origins(grid);
    let mut scores = 0;
    let mut ratings = 0;

    for s in starts.iter() {
        let (score, rating) = trailhead_total(grid, *s);
        scores += score;
        ratings += rating;
    }

    (scores, ratings)
}

fn origins(grid: &Grid) -> Vec<PointValue> {
    let mut starts = vec![];
    for y in 0..grid.ylen {
        for x in 0..grid.xlen {
            if grid.get(y, x) == 0 {
                starts.push((y, x, 0));
            }
        }
    }
    starts
}

fn trailhead_total(grid: &Grid, start: PointValue) -> (usize, usize) {
    let mut set = HashSet::new();
    let mut routes = 0;
    let mut deq = VecDeque::new();
    deq.push_back(start);

    while let Some((y, x, v)) = deq.pop_front() {
        for (dy, dx) in &TRANSLATIONS {
            let (ty, tx) = (y - dy, x - dx);

            if grid.out_of_bounds(ty, tx) {
                continue
            }

            let value = grid.get(ty, tx);

            if v + 1 != value {
                continue
            }

            deq.push_back((ty, tx, value));

            if value == 9 {
                routes += 1;
                set.insert((ty, tx));
                continue
            }
        }
    }

    (set.len(), routes)
}

#[test]
fn test_trailhead_totals() {
    let grid = parse("1");
    let (scores, _) = trailhead_totals(&grid);
    assert_eq!(scores, 1);

    let grid = parse("3");
    let (scores, _) = trailhead_totals(&grid);
    assert_eq!(scores, 2);

    let grid = parse("4");
    let (scores, _) = trailhead_totals(&grid);
    assert_eq!(scores, 4);

    let grid = parse("2");
    let (scores, ratings) = trailhead_totals(&grid);
    assert_eq!(scores, 36);
    assert_eq!(ratings, 81);
}
