use std::fs;

const NUMERIC: &'static str = "789456123 0A";
const DIRECTIONAL: &'static str = " ^A<v>";
const SIZE: isize = 3;

fn main() {
    println!("Hello, world!");
}

fn to_directional(numbers: &str) -> String {
    let mut directions = String::new();
    let mut start = pos(NUMERIC, 'A');

    for cs in numbers.chars() {
        let end = pos(NUMERIC, cs);
        let (y, x) = (start.0 - end.0, start.1 - end.1);

        let ns = if y > 0 { "^" } else { "v" };
        let ew = if x > 0 { "<" } else { ">" };

        let ya = y.abs() as usize;
        let xa = x.abs() as usize;

        directions.push_str(&ns.repeat(ya));
        directions.push_str(&ew.repeat(xa));
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
fn test_to_directional() {
    assert_eq!(to_directional("029A"), String::from("<A^A^^>AvvvA"));
}
