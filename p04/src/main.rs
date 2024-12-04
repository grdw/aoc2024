use std::fs;

type RawGrid = Vec<Vec<char>>;
type Point = (isize, isize);

struct Grid {
    vector: RawGrid,
    ymax: isize,
    xmax: isize,
    ylen: isize,
    xlen: isize
}

impl Grid {
    fn new(vector: RawGrid) -> Grid {
        let ymax = (vector.len() - 1) as isize;
        let xmax = (vector[0].len() - 1) as isize;
        let ylen = vector.len() as isize;
        let xlen = vector[0].len() as isize;

        Grid {vector, ymax, xmax, ylen, xlen}
    }

    fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        y < 0 || x < 0 || y > self.ymax || x > self.xmax
    }
}

const TRANSLATIONS: [Point; 8] = [
	(-1, -1), // TOP LEFT
	(-1, 0),  // TOP CENTRE
	(-1, 1),  // TOP RIGHT
	(0, -1),  // CENTRE LEFT
	(0, 1),   // CENTRE RIGHT
	(1, -1),  // BOTTOM LEFT
	(1, 0),   // BOTTOM CENTRE
	(1, 1)    // BOTTOM RIGHT
];

const CORNERS: [Point; 4] = [
	(-1, -1), // TOP LEFT
	(-1, 1),  // TOP RIGHT
	(1, -1),  // BOTTOM LEFT
	(1, 1)    // BOTTOM RIGHT
];

fn main() {
    let grid = parse("input");
    println!("p1 {}", xmas_count(&grid));
    println!("p2 {}", x_mas_count(&grid));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect();

    Grid::new(vector)
}

fn xmas_count(grid: &Grid) -> u32 {
    (0..grid.ylen).map(|y| {
        (0..grid.xlen).map(|x| {
            check_word(grid, &(y, x))
        }).sum::<u32>()
    }).sum()
}

fn check_word(grid: &Grid, point: &Point) -> u32 {
    let mut count = 0;

    for (dy, dx) in &TRANSLATIONS {
        let mut word = String::new();

        for i in 0..4 {
            let ddy = (dy * i) + point.0;
            let ddx = (dx * i) + point.1;

            if grid.out_of_bounds(ddy, ddx) { continue }

            let c = grid.vector[ddy as usize][ddx as usize];
            word.push(c);
        }

        if word == String::from("XMAS") {
            count += 1;
        }

        word.clear();
    }

    count
}

#[test]
fn test_xmas_counts() {
    let grid = parse("1");

    assert_eq!(xmas_count(&grid), 18)
}

fn x_mas_count(grid: &Grid) -> usize {
	let mut centres = vec![];

    for y in 0..grid.ylen {
        for x in 0..grid.xlen {
			if grid.vector[y as usize][x as usize] == 'A' {
				centres.push((y, x));
			}
        }
    }

	centres
        .iter()
        .filter(|centre| is_a_valid_x(&grid, centre))
        .count()
}

fn is_a_valid_x(grid: &Grid, point: &Point) -> bool {
	let valid_words = vec![
		"MSMS",
		"SSMM",
		"MMSS",
		"SMSM",
	];

	let mut word = String::new();
    for (dy, dx) in &CORNERS {
		let ddy = dy + point.0;
		let ddx = dx + point.1;

        if grid.out_of_bounds(ddy, ddx) { continue }

		let c = grid.vector[ddy as usize][ddx as usize];
		word.push(c);
    }

	valid_words.contains(&word.as_str())
}

#[test]
fn test_x_mas_counts() {
    let grid = parse("2");

    assert_eq!(x_mas_count(&grid), 9)
}

