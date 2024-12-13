use std::cmp::Ordering;
use std::fs;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let (patterns, designs) = parse("input");
    println!("p1 {}", possible_designs(&patterns, &designs));
    println!("p2 {}", total_design_count(&patterns, &designs));
}

fn parse(input: &'static str) -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string(input).unwrap();
    let (patterns_r, designs_r) = input.split_once("\n\n").unwrap();

    let patterns: Vec<String> = patterns_r
        .split(", ")
        .map(|p| p.to_string())
        .collect();

    let designs = designs_r
        .split_terminator("\n")
        .map(|d| d.to_string())
        .collect();

    (patterns, designs)
}

fn possible_designs(patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    designs.iter().filter(|&d| can_design(d, patterns)).count()
}

#[test]
fn test_possible_designs() {
    let (patterns, designs) = parse("1");
    assert_eq!(possible_designs(&patterns, &designs), 6);
}

fn can_design(design: &String, patterns: &Vec<String>) -> bool {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(0);

    while let Some(s) = queue.pop() {
        let slice = &design[0..s];

        if seen.contains(slice) {
            continue
        }

        if s == design.len() {
            return true
        }

        for p in patterns {
            let next = s + p.len();

            if next > design.len() {
                continue
            }

            if &design[s..next] == p.as_str() {
                queue.push(next);
            }
        }

        seen.insert(slice);
    }

    false
}

fn design_count(design: &String, patterns: &Vec<String>) -> usize {
    println!("{:?}", design);
    let mut possibilities = 0;

    let mut queue = BinaryHeap::new();
    //let mut seen = HashSet::new();
    queue.push(0);

    while let Some(s) = queue.pop() {
        //let slice = &design[0..s];

        //if seen.contains(slice) {
        //    continue
        //}

        if s == design.len() {
            possibilities += 1;
            println!("{:?}", possibilities);
        }

        for p in patterns {
            let next = s + p.len();

            if next > design.len() {
                continue
            }

            if &design[s..next] == p.as_str() {
                queue.push(next);
            }
        }

        //seen.insert(slice);
    }

    possibilities
}

fn total_design_count(patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    designs.iter().map(|d| design_count(&d, patterns)).sum()
}

#[test]
fn test_design_count() {
    let (patterns, designs) = parse("1");
    assert_eq!(total_design_count(&patterns, &designs), 16);
}
