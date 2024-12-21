use std::fs;
use std::collections::HashMap;

const NUMERIC: &'static str = "789456123 0A";
const DIRECTIONAL: &'static str = " ^A<v>";
const SIZE: isize = 3;

type Point = (isize, isize);

fn main() {
    println!("p1 {}", shortest_inputs("input", 2));
    println!("p2 {}", shortest_inputs("input", 25));
}

fn shortest_inputs(input: &'static str, n: usize) -> usize {
    let mut total = 0;
    let mut map = HashMap::new();

    for line in fs::read_to_string(input).unwrap().lines() {
        let mut line = line.to_string();
        let dir = to_my_input(&line, n, &mut map);

        line.retain(|x| x.is_numeric());
        let t = line.parse::<usize>().unwrap();

        total += t * dir;
    }
    total
}

#[test]
fn test_shortest_inputs() {
    assert_eq!(shortest_inputs("1", 2), 126384);
    assert_eq!(shortest_inputs("1", 25), 175396398527088);
}

// This is the Manhattan distance
fn manhattan_dist(s: &Point, e: &Point) -> usize {
    let dx = if s.0 < e.0 { e.0 - s.0 } else { s.0 - e.0 };
    let dy = if s.1 < e.1 { e.1 - s.1 } else { s.1 - e.1 };
    (dx + dy) as usize
}

fn to_my_input(
    numbers: &String,
    n: usize,
    map: &mut HashMap<String, usize>) -> usize {

    let mut len = 0;
    let dir = to_chunks(NUMERIC, numbers);

    // Warming the cash, counts as 1 step
    let mut map = HashMap::new();
    for d in &dir {
        let dir_chunks = to_chunks(DIRECTIONAL, d);

        for chunk in dir_chunks {
            map.entry(chunk).and_modify(|n| *n += 1).or_insert(1);
        }
    }

    for _ in 0..n-1 {
        let mut new_map = HashMap::new();
        for (key, value) in map {
            let chunks = to_chunks(DIRECTIONAL, &key);
            for c in chunks {
                new_map
                    .entry(c)
                    .and_modify(|n| *n += value)
                    .or_insert(value);
            }
        }

        map = new_map;
    }

    for (key, value) in &map {
        len += key.len() * value
    }

    len
}

#[test]
fn test_shortest_route_basic() {
    let input = "34".to_string();
    let dir = to_directional(NUMERIC, &input);
    assert_eq!(dir, String::from("^A<<^A"));
    let dir = to_directional(DIRECTIONAL, &dir);
    assert_eq!(dir, String::from("<A>Av<<AA>^A>A"));
}

fn to_chunks(keypad: &'static str, numbers: &String) -> Vec<String> {
    let mut result = vec![];
    let mut start = 'A';

    for cs in numbers.chars() {
        let q = to_result(keypad, start, cs);
        result.push(q);

        start = cs;
    }

    result
}

fn to_directional(keypad: &'static str, numbers: &str) -> String {
    let mut result = String::new();
    let mut start = 'A';

    for cs in numbers.chars() {
        let q = to_result(keypad, start, cs);
        result.push_str(&q);

        start = cs;
    }

    result
}

fn to_result(keypad: &'static str, start: char, cs: char) -> String {
    let mut result = String::new();
    let gap = pos(keypad, ' ');
    let sta = pos(keypad, start);
    let end = pos(keypad, cs);
    let (y, x) = (sta.0 - end.0, sta.1 - end.1);

    let ns = if y > 0 { b'^' } else { b'v' };
    let ew = if x > 0 { b'<' } else { b'>' };

    let ya = y.abs() as usize;
    let xa = x.abs() as usize;

    let py = String::from_utf8(vec![ns; ya]).unwrap();
    let px = String::from_utf8(vec![ew; xa]).unwrap();

    let d1 = (sta.0, end.1);
    let d2 = (end.0, sta.1);

    if d1 == gap {
        result.push_str(&py);
        result.push_str(&px);
    } else if d2 == gap {
        result.push_str(&px);
        result.push_str(&py);
    } else {
        let dir_y = pos(DIRECTIONAL, ns as char);
        let dir_x = pos(DIRECTIONAL, ew as char);
        let dir_a = pos(DIRECTIONAL, 'A');

        let my = manhattan_dist(&dir_y, &dir_a);
        let mx = manhattan_dist(&dir_x, &dir_a);

        if my > mx {
            result.push_str(&py);
            result.push_str(&px);
        } else {
            result.push_str(&px);
            result.push_str(&py);
        }
    }

    result.push('A');
    result
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
