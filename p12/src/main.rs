use std::fs;
use std::collections::{HashSet, VecDeque};

const TRANSLATIONS: [(char, isize, isize); 4] = [
    ('T', -2, 0),  // TOP
    ('L', 0, -2),  // LEFT
    ('R', 0, 2),   // RIGHT
    ('B', 2, 0),   // BOTTOM
];

const FENCE_TRANSLATIONS: [(char, isize, isize); 4] = [
    ('L', 0, -1),  // LEFT
    ('T', -1, 0),  // TOP
    ('R', 0, 1),   // RIGHT
    ('B', 1, 0),   // BOTTOM
];

type RawGarden = Vec<Vec<Spot>>;
type Area = (char, Vec<(isize, isize)>);

#[derive(Clone, Debug)]
enum Spot {
    Patch(char),
    Fence(char),
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
                    Spot::Fence(_) => print!("F"),
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
    println!("p2 {}", total_fencing_cost_with_discount(&patch));
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
                    'T' => fence_points.insert((dir, y - 1, x)),
                    'B' => fence_points.insert((dir, y + 1, x)),
                    'L' => fence_points.insert((dir, y, x - 1)),
                    'R' => fence_points.insert((dir, y, x + 1)),
                    _ => panic!("Invalid direction"),
                };
            }
        }
    }

    for (dir, fy, fx) in fence_points {
        garden.set(fy, fx, Spot::Fence(*dir));
    }
}

fn areas(garden: &Garden) -> Vec<Area> {
    let mut areas: Vec<Area> = vec![];
    let mut seen = HashSet::new();
    let mut vec = VecDeque::new();
    vec.push_back((' ', 1, 1));

    while let Some((prev_name, y, x)) = vec.pop_front() {
        if seen.contains(&(y, x)) {
            continue
        }

        let name = garden.name(y, x);

        if prev_name == name {
            let search = areas.iter_mut().rfind(|(n, _)| *n == name);

            if let Some((_, ref mut points)) = search {
                points.push((y, x));
            }
        } else {
            areas.push((name, vec![(y, x)]));
        }

        for (_, ty, tx) in &TRANSLATIONS {
            let (ey, ex) = (y + ty, x + tx);
            let new_name = garden.name(ey, ex);

            if new_name == ' ' {
                continue
            }

            if new_name == name {
                vec.push_front((name, ey, ex));
            } else {
                vec.push_back((name, ey, ex));
            }
        }

        seen.insert((y, x));
    }

    areas
}

fn total_fencing_cost(garden: &Garden) -> usize {
    let total_areas = areas(garden);
    let mut total_fencing = vec![];

    for (_, points) in &total_areas {
        let mut subtotal = 0;
        for (y, x) in points {
            for (_, ty, tx) in &FENCE_TRANSLATIONS {
                let (dy, dx) = (y + ty, x + tx);
                if let Spot::Fence(_) = garden.get(dy, dx) {
                    subtotal += 1;
                }
            }
        }
        total_fencing.push(subtotal);
    }

    (0..total_areas.len())
        .map(|i| (total_areas[i].1.len() * total_fencing[i]))
        .sum()
}

#[test]
fn test_fencing_cost_1() {
    let mut patch = parse("1");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 140);
}

#[test]
fn test_fencing_cost_2() {
    let mut patch = parse("2");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 772);
}

#[test]
fn test_fencing_cost_3() {
    let mut patch = parse("3");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 1930);
}

fn total_fencing_cost_with_discount(garden: &Garden) -> usize {
    let total_areas = areas(garden);
    let mut total_sides = vec![];

    for (name, points) in &total_areas {
        let mut subtotal = 0;
        let mut fences = vec![];

        for (y, x) in points {
            for (dir, ty, tx) in &FENCE_TRANSLATIONS {
                let (dy, dx) = (y + ty, x + tx);
                if let Spot::Fence(_) = garden.get(dy, dx) {
                    fences.push((*dir, y, x));
                }
            }
        }

        fences.sort_by_key(|&(_, y, x)| (y, x));

        println!("{:?}", fences);

        for i in 0..fences.len() {
            let j = (i + 1) % fences.len();
            let corner = format!("{}{}", fences[i].0, fences[j].0);

            print!("{}", corner);

            match corner.as_str() {
                "LT" | "TL" | "LB" | "BL" | "RT" | "TR" | "BR" | "RB" => {
                    println!(".");
                    subtotal += 1;
                },
                _ => ()
            }
        }
        println!("== {} {:?}", name, subtotal);
        total_sides.push(subtotal);
    }

    (0..total_areas.len())
        .map(|i| (total_areas[i].1.len() * total_sides[i]))
        .sum()
}

#[test]
fn test_fencing_cost_with_discount_1() {
    let mut patch = parse("1");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost_with_discount(&patch), 80);
}

#[test]
fn test_fencing_cost_with_discount_2() {
    let mut patch = parse("2");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost_with_discount(&patch), 436);
}

#[test]
fn test_fencing_cost_with_discount_3() {
    let mut patch = parse("3");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost_with_discount(&patch), 1206);
}
