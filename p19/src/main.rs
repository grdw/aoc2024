use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let (patterns, designs) = parse("input");
    println!("p1 {}", possible_designs(&patterns, &designs));
    println!("p2 {}", total_design_count(&patterns, &designs));
}

fn parse(input: &'static str) -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string(input).unwrap();
    let (patterns_r, designs_r) = input.split_once("\n\n").unwrap();

    let patterns = patterns_r
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
    let mut queue = vec![];
    let mut seen = HashSet::new();
    queue.push(&design[..]);

    while let Some(s) = queue.pop() {
        if s.is_empty() {
            return true
        }

        if seen.contains(&s) {
            continue
        }

        for p in patterns {
            if let Some(n) = s.strip_prefix(p) {
                queue.push(n)
            }
        }

        seen.insert(s);
    }

    false
}

fn total_design_count(patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    let mut memo: HashMap<&str, usize> = HashMap::new();
    let max = patterns.iter().map(|n| n.len()).max().unwrap();
    memo.insert("", 1);

    designs
        .iter()
        .map(|design| design_count(design.as_str(), patterns, max, &mut memo))
        .sum()
}

fn design_count<'a>(
    design: &'a str,
    patterns: &Vec<String>,
    max: usize,
    memo: &mut HashMap<&'a str, usize>) -> usize {

    if memo.contains_key(design) {
        return memo[design]
    }

    let mut count = 0;

    for i in 0..design.len().min(max) {
        let (prefix, suffix) = design.split_at(i + 1);
        if patterns.contains(&prefix.to_string()) {
            count += design_count(&suffix, patterns, max, memo);
        }
    }

    memo.insert(design, count);
    count
}

#[test]
fn test_design_count() {
    let (patterns, designs) = parse("1");

    assert_eq!(total_design_count(&patterns, &designs), 16);
}
