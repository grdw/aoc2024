use std::fs;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let (patterns, designs) = parse("input");
    println!("p1 {}", possible_designs(&patterns, &designs));
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
    designs.iter().filter(|&d| can_make_design(d, patterns)).count()
}

fn can_make_design(design: &String, patterns: &Vec<String>) -> bool {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(0);

    while let Some(s) = queue.pop() {
        if seen.contains(&design[0..s]) {
            continue
        }
        if s == design.len() {
            return true
        }

        for p in patterns {
            let e = s + p.len();

            if e > design.len() {
                continue
            }

            if &design[s..e] == p.as_str() {
                queue.push(e);
            }
        }

        seen.insert(&design[0..s]);
    }
    false
}

#[test]
fn test_possible_designs() {
    let (patterns, designs) = parse("1");
    assert_eq!(possible_designs(&patterns, &designs), 6);
}
