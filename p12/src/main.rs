use std::fs;

const TRANSLATIONS: [(char, isize, isize); 4] = [
    ('T', -2, 0),  // TOP
    ('L', 0, -2),  // LEFT
    ('R', 0, 2),   // RIGHT
    ('B', 2, 0),   // BOTTOM
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
}

fn parse(input: &'static str) -> Garden {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Garden::new(vector)
}

fn main() {
    println!("Hello, world!");
}

fn fence_off(garden: &mut Garden) {
    let mut fence_points = vec![];

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

                match dir {
                    'T' => fence_points.push((y + 1, x)),
                    'B' => fence_points.push((y - 1, x)),
                    'L' => fence_points.push((y, x - 1)),
                    'R' => fence_points.push((y, x + 1)),
                    _ => panic!("Invalid direction"),
                }
            }
        }
    }

    for (fy, fx) in fence_points {
        garden.set(fy, fx, Spot::Fence);
    }
}

fn total_fencing_cost(garden: &Garden) -> usize {
    0
}

#[test]
fn test_fencing() {
    let mut patch = parse("1");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 140);
    let mut patch = parse("2");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 772);
    let mut patch = parse("3");
    fence_off(&mut patch);
    assert_eq!(total_fencing_cost(&patch), 1930);
}
