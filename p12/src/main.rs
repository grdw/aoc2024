use std::fs;
use std::cmp;
use std::collections::{HashSet, HashMap, VecDeque};

const TRANSLATIONS: [(char, isize, isize); 4] = [
    ('T', -2, 0),  // TOP
    ('R', 0, 2),   // RIGHT
    ('B', 2, 0),   // BOTTOM
    ('L', 0, -2),  // LEFT
];

type RawGarden = Vec<Vec<Spot>>;
type Point = (isize, isize);
type DirPoint = (char, isize, isize);
type Area = (char, Vec<Point>);

#[derive(Clone, Debug)]
enum Spot {
    Patch(char),
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
    let patch = parse("input");
    println!("p1 {}", total_fencing_cost(&patch));
    println!("p2 {}", total_fencing_cost_with_discount(&patch));
}

fn fence_off(areas: &Vec<Area>) -> Vec<Vec<DirPoint>> {
    let mut map = vec![];

    for i in 0..areas.len() {
        let (_name, points) = &areas[i];
        let mut list = vec![];

        for (y, x) in points.iter() {
            for (dir, ty, tx) in &TRANSLATIONS {
                let (dy, dx) = (ty + y, tx + x);

                if points.iter().any(|&(py, px)| py == dy && px == dx) {
                    continue
                }

                let point = match dir {
                    'T' => (*dir, *y - 1, *x),
                    'B' => (*dir, *y + 1, *x),
                    'L' => (*dir, *y, *x - 1),
                    'R' => (*dir, *y, *x + 1),
                    _ => panic!("Invaid direction"),
                };

                list.push(point);
            }
        }
        map.push(list);
    }

    map
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
    let fences = fence_off(&total_areas);

    (0..total_areas.len())
        .map(|i| (total_areas[i].1.len() * fences[i].len()))
        .sum()
}

#[test]
fn test_fencing_cost_1() {
    let patch = parse("1");
    assert_eq!(total_fencing_cost(&patch), 140);
}

#[test]
fn test_fencing_cost_2() {
    let patch = parse("2");
    assert_eq!(total_fencing_cost(&patch), 772);
}

#[test]
fn test_fencing_cost_3() {
    let patch = parse("3");
    assert_eq!(total_fencing_cost(&patch), 1930);
}

fn total_fencing_cost_with_discount(garden: &Garden) -> usize {
    let total_areas = areas(garden);
    let fences = fence_off(&total_areas);
    let mut total_sides = vec![];

    for i in 0..total_areas.len() {
        let mut po: HashMap<Point, usize> = HashMap::new();

        for k in 0..fences[i].len() {
            for l in (k + 1)..fences[i].len() {
                let (_, ay, ax) = &fences[i][k];
                let (_, by, bx) = &fences[i][l];
                let (dy, dx) = ((ay - by).abs(), (ax - bx).abs());

                if !(dy == 1 && dx == 1) {
                    continue
                }

                let (mut ny, mut nx) = (0, 0);
                if garden.name(*ay, *bx) == ' ' {
                    (ny, nx) = (*ay, *bx);
                } else if garden.name(*by, *ax) == ' ' {
                    (ny, nx) = (*by, *ax);
                }

                po.entry((ny, nx))
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }

        // This is so disgusting hahaha
        let k = po.values().map(|n| cmp::min(n, &2)).sum::<usize>();

        total_sides.push(k);
    }

    (0..total_areas.len())
        .map(|i| (total_areas[i].1.len() * total_sides[i]))
        .sum()
}

#[test]
fn test_fencing_cost_with_discount_1() {
    let patch = parse("1");
    assert_eq!(total_fencing_cost_with_discount(&patch), 80);
}

#[test]
fn test_fencing_cost_with_discount_2() {
    let patch = parse("2");
    assert_eq!(total_fencing_cost_with_discount(&patch), 436);
}

#[test]
fn test_fencing_cost_with_discount_3() {
    let patch = parse("3");
    assert_eq!(total_fencing_cost_with_discount(&patch), 1206);
}

#[test]
fn test_fencing_cost_with_discount_4() {
    let patch = parse("4");
    assert_eq!(total_fencing_cost_with_discount(&patch), 236);
}

#[test]
fn test_fencing_cost_with_discount_5() {
    let patch = parse("5");
    assert_eq!(total_fencing_cost_with_discount(&patch), 368);
}
