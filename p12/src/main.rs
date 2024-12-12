use std::fs;
use std::collections::{HashMap, HashSet};

const TRANSLATIONS: [(char, isize, isize); 4] = [
    ('T', -2, 0),  // TOP
    ('L', 0, -2),  // LEFT
    ('R', 0, 2),   // RIGHT
    ('B', 2, 0),   // BOTTOM
];

const FENCE_TRANSLATIONS: [(isize, isize); 4] = [
    (-1, 0),  // TOP
    (0, -1),  // LEFT
    (0, 1),   // RIGHT
    (1, 0),   // BOTTOM
];

type RawGarden = Vec<Vec<Spot>>;

#[derive(Clone, Debug)]
enum Spot {
    Patch(char),
    Fence,
    Empty
}

#[derive(Debug)]
struct Garden {
    vector: RawGarden,
    ylen: isize,
    xlen: isize
}

impl Garden {
    fn new(raw_vector: Vec<Vec<char>>) -> Garden {
        let vector = Self::expand(raw_vector);
        let ylen = vector.len() as isize;
        let xlen = vector[0].len() as isize;

        Garden {vector, ylen, xlen}
    }

    fn expand(vector: Vec<Vec<char>>) -> RawGarden {
        let mut v = vec![];
        let ylen = vector.len();
        let xlen = vector[0].len();

        for y in 0..ylen {
            let mut row = vec![Spot::Empty];
            for x in 0..xlen {
                row.push(Spot::Patch(vector[y][x]));
                row.push(Spot::Empty);
            }
            let empty_row = vec![Spot::Empty; row.len()];
            v.push(empty_row);
            v.push(row);
        }

        let bottom_row = vec![Spot::Empty; v[0].len()];
        v.push(bottom_row);

        v
    }

    fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        y < 0 || x < 0 || y >= self.ylen || x >= self.xlen
    }

    fn get(&self, y: isize, x: isize) -> &Spot {
        &self.vector[y as usize][x as usize]
    }

    fn set(&mut self, y: isize, x: isize, spot: Spot) {
        self.vector[y as usize][x as usize] = spot;
    }

    fn name(&self, y: isize, x: isize) -> char {
        if self.out_of_bounds(y, x) {
            return ' '
        }

        match self.get(y, x) {
            Spot::Patch(ac) => *ac,
            _ => ' '
        }
    }

    fn debug(&self) {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                match self.get(y, x) {
                    Spot::Empty => print!(" "),
                    Spot::Fence => print!("F"),
                    Spot::Patch(c) => print!("{}", c)
                }
            }
            println!("");
        }
    }
}

fn parse(input: &'static str) -> Garden {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Garden::new(vector)
}

fn main() {
    let mut patch = parse("input");
    fence_off(&mut patch);
    println!("p1 {}", total_fencing_cost(&patch));
}

fn fence_off(garden: &mut Garden) {
    let mut fence_points = HashSet::new();

    for y in 0..garden.ylen {
        for x in 0..garden.xlen {
            let ac = garden.name(y, x);

            if ac == ' ' {
                continue
            }

            for (dir, ty, tx) in &TRANSLATIONS {
                let (dy, dx) = (ty + y, tx + x);

                let bc = garden.name(dy, dx);

                if ac == bc { continue }

                let _ = match dir {
                    'T' => fence_points.insert((y - 1, x)),
                    'B' => fence_points.insert((y + 1, x)),
                    'L' => fence_points.insert((y, x - 1)),
                    'R' => fence_points.insert((y, x + 1)),
                    _ => panic!("Invalid direction"),
                };
            }
        }
    }

    for (fy, fx) in fence_points {
        garden.set(fy, fx, Spot::Fence);
    }

    garden.debug()
}

fn total_fencing_cost(garden: &Garden) -> usize {
    let mut total_area = HashMap::new();
    let mut total_fencing = HashMap::new();

    for y in 0..garden.ylen {
        for x in 0..garden.xlen {
            let name = garden.name(y, x);
            if name == ' ' {
                continue
            }

            total_area
                .entry(name)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            for (ty, tx) in &FENCE_TRANSLATIONS {
                let (dy, dx) = (y + ty, x + tx);
                if let Spot::Fence = garden.get(dy, dx) {
                    total_fencing
                        .entry(name)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
    }

    total_area
        .iter()
        .map(|(key, area)| area * total_fencing[key])
        .sum()
}

#[test]
fn test_fencing_1() {
    let mut patch = parse("1");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 140);
}

#[test]
fn test_fencing_2() {
    let mut patch = parse("2");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 772);

    let mut patch = parse("3");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 1930);
}
