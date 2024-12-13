use std::fs;
use std::collections::HashMap;

const ADD: isize = 10_000_000_000_000;

type Point = (isize, isize);
type PointC = (isize, isize, isize);
type ClawMachine = (Point, Vec<PointC>);

fn main() {
    let claw_machines = parse("input");
    println!("p1: {:?}", minimum_tokens(&claw_machines, 0));
    println!("p2: {:?}", minimum_tokens(&claw_machines, ADD));
}

fn parse(input: &'static str) -> Vec<ClawMachine> {
    let mut claw_machines = vec![];
    let raw_input = fs::read_to_string(input).unwrap();

    let costs = HashMap::from([
        ('A', 3),
        ('B', 1),
    ]);

    for claw in raw_input.split("\n\n") {
        let mut prize = (0, 0);
        let mut buttons = vec![];
        for line in claw.split_terminator("\n") {
            let coords = parse_line(line);
            if line.starts_with("Button") {
                let (_, r) = line.split_once("Button ").unwrap();
                let name = r.chars().nth(0).unwrap();

                let coords_with_name = (
                    coords.0,
                    coords.1,
                    costs[&name]
                );

                buttons.push(coords_with_name);
            } else {
                prize = coords;
            }
        }
        claw_machines.push((prize, buttons));
    }
    claw_machines
}

fn parse_line(line: &str) -> (isize, isize) {
    let mut coords = (0, 0);
    let (l, y) = line.split_once(", ").unwrap();
    let (_, x) = l.split_once(": ").unwrap();
    coords.0 = y[2..].parse::<isize>().unwrap();
    coords.1 = x[2..].parse::<isize>().unwrap();
    coords
}

fn minimum_tokens(claw_machines: &Vec<ClawMachine>, a: isize) -> isize {
    let mut total = 0;
    for (prize, buttons) in claw_machines.iter() {
        let (y, x) = *prize;
        let new_prize = (y + a, x + a);
        if let Some(n) = token_balance(&new_prize, buttons) {
            total += n;
        }
    }
    total
}

#[test]
fn test_minimum_tokens() {
    let claw_machines = parse("1");

    assert_eq!(minimum_tokens(&claw_machines, 0), 480);
}

fn token_balance(prize: &Point, buttons: &Vec<PointC>) -> Option<isize> {
    let (ay, ax, _) = buttons[0];
    let (by, bx, _) = buttons[1];
    let (ty, tx) = *prize;

    let b = (ax * ty - ay * tx) / (ax * by - ay * bx);
    let a = (tx - bx * b) / ax;
    if ax * a + bx * b == tx && ay * a + by * b == ty {
        Some(a * 3 + b)
    } else {
        None
    }
}

#[test]
fn test_token_balance() {
    let claw_machines = parse("1");

    let (goal, points) = &claw_machines[0];
    assert_eq!(token_balance(goal, points), Some(280));

    let (goal, points) = &claw_machines[0];
    let new_goal = (goal.0 + ADD, goal.1 + ADD);
    assert_eq!(token_balance(&new_goal, points), None);

    let (goal, points) = &claw_machines[1];
    assert_eq!(token_balance(goal, points), None);
}

