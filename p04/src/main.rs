use std::fs;

type Grid = Vec<Vec<char>>;
type Point = (isize, isize);

fn main() {
    let grid = parse("input");
    println!("p1 {}", xmas_count(&grid));
}

fn parse(input: &'static str) -> Grid {
    fs::read_to_string(input).unwrap().lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn xmas_count(grid: &Grid) -> u32 {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            total += check_dir(grid, &(y as isize, x as isize));
        }
    }

    total
}

fn check_dir(grid: &Grid, point: &Point) -> u32 {
    let mut count = 0;
    let ymax = (grid.len() - 1) as isize;
    let xmax = (grid[0].len() - 1) as isize;

    let diffs = vec![
        (-1, -1), // TOP LEFT
        (-1, 0),  // TOP CENTRE
        (-1, 1),  // TOP RIGHT
        (0, -1),  // CENTRE LEFT
        (0, 1),   // CENTRE RIGHT
        (1, -1),  // BOTTOM LEFT
        (1, 0),   // BOTTOM CENTRE
        (1, 1)    // BOTTOM RIGHT
    ];

    for (dy, dx) in &diffs {
        let mut word = String::new();

        for i in 0..4 {
            let ddy = (dy * i) + point.0;
            let ddx = (dx * i) + point.1;

            if ddy < 0 || ddx < 0 { continue }
            if ddy > ymax || ddx > xmax { continue }

            let c = grid[ddy as usize][ddx as usize];
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

