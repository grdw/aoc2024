use std::fs;

const NUMERIC: &'static str = "789456123 0A";
const DIRECTIONAL: &'static str = " ^A<v>";
const SIZE: isize = 3;

fn main() {
    println!("Hello, world!");
}

fn shortest_inputs(input: &'static str) -> usize {
    let mut total = 0;

    for line in fs::read_to_string(input).unwrap().lines() {
        let mut line = line.to_string();
        let mut dir = to_directional(NUMERIC, &line);
        println!("{:?}", dir);
        dir = to_directional(DIRECTIONAL, &dir);
        println!("{:?}", dir);
        dir = to_directional(DIRECTIONAL, &dir);
        println!("{:?}", dir);

        println!("{} {}", line, dir.len());
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

fn to_directional(keypad: &'static str, numbers: &String) -> String {
    let mut directions = String::new();
    let mut start = pos(keypad, 'A');

    for cs in numbers.chars() {
        let end = pos(keypad, cs);
        let (y, x) = (start.0 - end.0, start.1 - end.1);

        let ns = if y > 0 { "^" } else { "v" };
        let ew = if x > 0 { "<" } else { ">" };

        let ya = y.abs() as usize;
        let xa = x.abs() as usize;

        let d1 = (end.0, start.1);

        if keypad.chars().nth(to_i(d1)) == Some(' ') {
            directions.push_str(&ew.repeat(xa));
            directions.push_str(&ns.repeat(ya));
        } else {
            directions.push_str(&ns.repeat(ya));
            directions.push_str(&ew.repeat(xa));
        }

        directions.push('A');

        start = end;
    }

    directions
}

fn to_i(pos: (isize, isize)) -> usize {
    ((pos.0 * SIZE) + pos.1) as usize
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
        String::from("<A^A^^>AvvvA")
    );

    assert_eq!(
        to_directional(DIRECTIONAL, &to_directional(NUMERIC, &input)),
        String::from("v<<A>>^A<A>A<AAv>A^Av<AAA^>A")
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
