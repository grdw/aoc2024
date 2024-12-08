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
    let antennas = get_antennas(&grid);
    println!("p1 {}", uniq_antinodes(&grid, &antennas));
    println!("p2 {}", uniq_resonating_antinodes(&grid, &antennas));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn uniq_antinodes(grid: &Grid, antennas: &Antennas) -> usize {
    let mut set = HashSet::new();

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
fn test_uniq_antinodes() {
    let grid = parse("1");
    let antennas = get_antennas(&grid);

    assert_eq!(uniq_antinodes(&grid, &antennas), 14)
}

fn uniq_resonating_antinodes(grid: &Grid, antennas: &Antennas) -> usize {
    let mut set = HashSet::new();

    for (_, ants) in antennas {
        for i in 0..ants.len() {
            for j in (i + 1)..ants.len() {
                let (ky, kx) = ants[j];
                let (ly, lx) = ants[i];
                let (dy, dx) = (ky - ly, kx - lx);

                let (mut ay, mut ax) = (ky, kx);
                let (mut by, mut bx) = (ly, lx);

                while !grid.out_of_bounds(&(ay, ax)) {
                    set.insert((ay, ax));
                    ay += dy;
                    ax += dx;
                }

                while !grid.out_of_bounds(&(by, bx)) {
                    set.insert((by, bx));
                    by -= dy;
                    bx -= dx;
                }
            }
        }
    }

    set.len()
}

#[test]
fn test_uniq_resonating_antinodes_1() {
    let grid = parse("1");
    let antennas = get_antennas(&grid);

    assert_eq!(uniq_resonating_antinodes(&grid, &antennas), 34)
}

#[test]
fn test_uniq_resonating_antinodes_2() {
    let grid = parse("2");
    let antennas = get_antennas(&grid);

    assert_eq!(uniq_resonating_antinodes(&grid, &antennas), 9);
}

fn get_antennas(grid: &Grid) -> Antennas {
    let mut antennas: Antennas = HashMap::new();

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

    antennas
}
