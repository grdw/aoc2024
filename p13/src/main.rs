use std::fs;
use std::collections::HashMap;

type Point = (usize, usize);
type PointWithCost = (usize, usize, usize);
type ClawMachine = (Point, Vec<PointWithCost>);

fn main() {
    let claw_machines = parse("input");
    println!("p1: {:?}", minimum_tokens(&claw_machines));
    //println!("Part 2: {:?}", part2("input"));
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

fn parse_line(line: &str) -> (usize, usize) {
    let mut coords = (0, 0);
    let (l, y) = line.split_once(", ").unwrap();
    let (_, x) = l.split_once(": ").unwrap();
    coords.0 = y[2..].parse::<usize>().unwrap();
    coords.1 = x[2..].parse::<usize>().unwrap();
    coords
}

fn minimum_tokens(claw_machines: &Vec<ClawMachine>) -> usize {
    let mut total = 0;
    for (prize, buttons) in claw_machines.iter() {
        if let Some(n) = token_balance(prize, buttons) {
            total += n;
        }
    }
    total
}

#[test]
fn test_minimum_tokens() {
    let claw_machines = parse("1");

    assert_eq!(minimum_tokens(&claw_machines), 480);
}

fn token_balance(prize: &Point, buttons: &Vec<PointWithCost>) -> Option<usize> {
    let mut button_presses = 1;

    loop {
        let mut a = button_presses;

        while a > 0 {
            let b = button_presses - a;

            let ty = (buttons[0].0 * a) + (buttons[1].0 * b);
            let tx = (buttons[0].1 * a) + (buttons[1].1 * b);

            if ty == prize.0 && tx == prize.1 {
                return Some((buttons[0].2 * a) + (buttons[1].2 * b));
            } else if ty > prize.0 && tx > prize.1 {
                return None;
            }

            a -= 1;
        }

        button_presses += 1;
    }

}

#[test]
fn test_token_balance() {
    let claw_machines = parse("1");

    let (goal, points) = &claw_machines[0];

    assert_eq!(token_balance(goal, points), Some(280));

    let (goal, points) = &claw_machines[1];

    assert_eq!(token_balance(goal, points), None);
}

