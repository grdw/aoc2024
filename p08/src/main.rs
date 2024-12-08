use std::fs;
use std::collections::{HashSet, HashMap};

type RawGrid = Vec<Vec<char>>;
type Point = (isize, isize);
type Antennas = HashMap<char, Vec<Point>>;

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
}

fn main() {
    let grid = parse("input");
    println!("p1 {}", unique_antinodes(&grid))
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn unique_antinodes(grid: &Grid) -> usize {
    let mut antennas: Antennas = HashMap::new();
    let mut set = HashSet::new();

    for y in 0..grid.ylen {
        for x in 0..grid.xlen {
            let c = grid.get(&(y, x));
            if c == '.'{
                continue
            }

            antennas
                .entry(c)
                .and_modify(|n| n.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    for (_, ants) in antennas {
        for i in 0..ants.len() {
            for j in (i + 1)..ants.len() {
                let (ky, kx) = ants[j];
                let (ly, lx) = ants[i];
                let (dy, dx) = (ky - ly, kx - lx);

                let a1 = (ky + dy, kx + dx);
                let b1 = (ly - dy, lx - dx);

                if !grid.out_of_bounds(&a1) {
                    set.insert(a1);
                }

                if !grid.out_of_bounds(&b1) {
                    set.insert(b1);
                }
            }
        }
    }

    set.len()
}

#[test]
fn test_unique_antinodes() {
    let grid = parse("1");
    assert_eq!(unique_antinodes(&grid), 14)
}
