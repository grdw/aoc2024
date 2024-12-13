use std::fs;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const MAX_COST: usize = 400;

type Point = (usize, usize);
type PointWithName = (usize, usize, char);
type ClawMachine = (Point, Vec<PointWithName>);

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let claw_machines = parse("input");
    println!("p1: {:?}", minimum_tokens(&claw_machines));
    //println!("Part 2: {:?}", part2("input"));
}

fn parse(input: &'static str) -> Vec<ClawMachine> {
    let mut claw_machines = vec![];
    let raw_input = fs::read_to_string(input).unwrap();

    for claw in raw_input.split("\n\n") {
        let mut prize = (0, 0);
        let mut buttons = vec![];
        for line in claw.split_terminator("\n") {
            let coords = parse_line(line);
            if line.starts_with("Button") {
                let (_, r) = line.split_once("Button ").unwrap();
                let coords_with_name = (
                    coords.0,
                    coords.1,
                    r.chars().nth(0).unwrap()
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
        if let Some(n) = dijkstra(prize, buttons) {
            total += n;
        }
    }
    total
}

fn dijkstra(goal: &Point, buttons: &Vec<PointWithName>) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();
    let start = (0, 0);

    let costs = HashMap::from([
        ('A', 3),
        ('B', 1),
    ]);

    heap.push(State {
        cost: 0,
        position: start
    });

    while let Some(State { cost, position }) = heap.pop() {
        if let Some(&visited_cost) = dist.get(&position) {
            if cost >= visited_cost {
                continue;
            }
        }

        dist.insert(position, cost);

        if &position == goal {
            return Some(cost);
        }

        if cost > MAX_COST {
            continue
        }

        for (ty, tx, name) in buttons {
            let next = State {
                cost: cost + costs[name],
                position: (position.0 + ty, position.1 + tx)
            };

            heap.push(next);
        }
    }

    None
}

#[test]
fn test_minimum_tokens() {
    let claw_machines = parse("1");

    assert_eq!(minimum_tokens(&claw_machines), 480);
}
