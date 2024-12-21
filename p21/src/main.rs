use std::fs;
use std::collections::HashMap;

const NUMERIC: &'static str = "789456123 0A";
const DIRECTIONAL: &'static str = " ^A<v>";
const SIZE: isize = 3;

type Point = (isize, isize);

fn main() {
    println!("p1 {}", shortest_inputs("input", 2));
    //println!("p2 {}", shortest_inputs("input", 25));
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
    assert_eq!(shortest_inputs("1", 25), 126384);
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
    map: &mut HashMap<String, String>) -> usize {

    let mut len = 0;
    let dir = to_chunks(NUMERIC, numbers);
    let mut queue = vec![];

    for d in dir.into_iter() {
        queue.insert(0, (d, 0));
    }

    while let Some((d, depth)) = queue.pop() {
        if depth == n {
            len += d.len();
            continue
        }

        if let Some(cached_chunks) = map.get(&d) {
            for chunk in cached_chunks.split_inclusive('A') {
                queue.push((chunk.to_string(), depth + 1));
            }

            continue
        }

        let chunks = to_directional(DIRECTIONAL, &d);
        map.insert(d.to_string(), chunks.clone());
        for chunk in chunks.split_inclusive('A') {
            let c = chunk.to_string();
            queue.push((c, depth + 1));
        }

    }
    println!("{:?}", len);

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
        let ((c1, n1), (c2, n2)) = to_result(keypad, start, cs);
        println!("{} {} | {} {}", c1, n1, c2, n2);

        start = cs;
    }

    result
}

fn to_directional(keypad: &'static str, numbers: &str) -> String {
    let mut result = String::new();
    let mut start = 'A';

    for cs in numbers.chars() {
        let ((c1, n1), (c2, n2)) = to_result(keypad, start, cs);
        let a = vec![c1; n1];
        let b = vec![c2; n2];
        let c: String = a.iter().collect();
        let d: String = b.iter().collect();
        result.push_str(&c);
        result.push_str(&d);
        result.push('A');

        start = cs;
    }

    result
}

fn to_result(keypad: &'static str, start: char, cs: char) -> (
    (char, usize), (char, usize)
) {
    let gap = pos(keypad, ' ');
    let sta = pos(keypad, start);
    let end = pos(keypad, cs);
    let (y, x) = (sta.0 - end.0, sta.1 - end.1);

    let ns = if y > 0 { '^' } else { 'v' };
    let ew = if x > 0 { '<' } else { '>' };

    let ya = y.abs() as usize;
    let xa = x.abs() as usize;

    let d1 = (sta.0, end.1);
    let d2 = (end.0, sta.1);

    if d1 == gap {
        return ((ns, ya), (ew, xa))
    } else if d2 == gap {
        return ((ew, xa), (ns, ya))
    } else {
        let dir_y = pos(DIRECTIONAL, ns);
        let dir_x = pos(DIRECTIONAL, ew);
        let dir_a = pos(DIRECTIONAL, 'A');

        let my = manhattan_dist(&dir_y, &dir_a);
        let mx = manhattan_dist(&dir_x, &dir_a);

        if my > mx {
            return ((ns, ya), (ew, xa))
        } else {
            return ((ew, xa), (ns, ya))
        }
    }
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
