use std::fs;

const NUMERIC: &'static str = "789456123 0A";
const DIRECTIONAL: &'static str = " ^A<v>";
const SIZE: isize = 3;

type Point = (isize, isize);

fn main() {
    println!("p1 {}", shortest_inputs("input"));
}

fn shortest_inputs(input: &'static str) -> usize {
    let mut total = 0;

    for line in fs::read_to_string(input).unwrap().lines() {
        let mut line = line.to_string();
        let dir = to_my_input(&line);

        line.retain(|x| x.is_numeric());
        let t = line.parse::<usize>().unwrap();

        total += t * dir.len()
    }
    total
}

#[test]
fn test_shortest_inputs() {
    assert_eq!(shortest_inputs("1"), 126384);
}

// This is the Manhattan distance
fn manhattan_dist(s: &Point, e: &Point) -> usize {
    let dx = if s.0 < e.0 { e.0 - s.0 } else { s.0 - e.0 };
    let dy = if s.1 < e.1 { e.1 - s.1 } else { s.1 - e.1 };
    (dx + dy) as usize
}

fn to_my_input(numbers: &String) -> String {
    let mut dir = to_directional(NUMERIC, numbers);
    dir = to_directional(DIRECTIONAL, &dir);
    dir = to_directional(DIRECTIONAL, &dir);
    dir
}

#[test]
fn test_shortest_route_basic() {
    let input = "34".to_string();
    let dir = to_directional(NUMERIC, &input);
    assert_eq!(dir, String::from("^A<<^A"));
    let dir = to_directional(DIRECTIONAL, &dir);
    assert_eq!(dir, String::from("<A>Av<<AA>^A>A"));
}

fn to_directional(keypad: &'static str, numbers: &String) -> String {
    let mut directions = String::new();
    let mut start = pos(keypad, 'A');
    let gap = pos(keypad, ' ');

    for cs in numbers.chars() {
        let end = pos(keypad, cs);
        let (y, x) = (start.0 - end.0, start.1 - end.1);

        let ns = if y > 0 { b'^' } else { b'v' };
        let ew = if x > 0 { b'<' } else { b'>' };

        let ya = y.abs() as usize;
        let xa = x.abs() as usize;

        let py = String::from_utf8(vec![ns; ya]).unwrap();
        let px = String::from_utf8(vec![ew; xa]).unwrap();

        let d1 = (start.0, end.1);
        let d2 = (end.0, start.1);

        if d1 == gap {
            directions.push_str(&py);
            directions.push_str(&px);
        } else if d2 == gap {
            directions.push_str(&px);
            directions.push_str(&py);
        } else {
            let dir_y = pos(DIRECTIONAL, ns as char);
            let dir_x = pos(DIRECTIONAL, ew as char);
            let dir_a = pos(DIRECTIONAL, 'A');

            let my = manhattan_dist(&dir_y, &dir_a);
            let mx = manhattan_dist(&dir_x, &dir_a);

            if my > mx {
                directions.push_str(&py);
                directions.push_str(&px);
            } else {
                directions.push_str(&px);
                directions.push_str(&py);
            }
        }

        directions.push('A');

        start = end;
    }

    directions
}

fn pos(keypad: &'static str, lookup: char) -> (isize, isize) {
    let p = (keypad.chars().position(|c| c == lookup).unwrap()) as isize;
    let y = p.div_euclid(SIZE);
    let x = p.rem_euclid(SIZE);
    (y, x)
}

#[test]
fn test_skip_gaps() {
    let input = "01".to_string();
    assert_eq!(to_directional(&NUMERIC, &input), String::from("<A^<A"));

    let input = "70".to_string();
    assert_eq!(
        to_directional(&NUMERIC, &input),
        String::from("^^^<<A>vvvA")
    );
}

#[test]
fn test_to_directional() {
    let input = "029A".to_string();

    assert_eq!(
        to_directional(NUMERIC, &input),
        String::from("<A^A>^^AvvvA")
    );

    assert_eq!(
        to_directional(
            DIRECTIONAL,
            &to_directional(NUMERIC, &input)
        ).len(),
        28
    );

    assert_eq!(
        to_directional(
            DIRECTIONAL,
            &to_directional(
                DIRECTIONAL,
                &to_directional(
                    NUMERIC,
                    &input
                )
            )
        ).len(),
        68
    );
}
