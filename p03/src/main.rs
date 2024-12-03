use std::fs;
use std::ops::Range;
use regex::Regex;

fn main() {
    let shopkeeper_data = fs::read_to_string("input").unwrap();
    let skips = parse_skips(&shopkeeper_data);
    println!("p1 {}", multiply(&shopkeeper_data, &vec![]));
    println!("p2 {}", multiply(&shopkeeper_data, &skips));
}

fn parse_skips(input: &String) -> Vec<Range<usize>> {
    let skips_re = Regex::new(r"don't\(\).*do\(\)").unwrap();
    skips_re
        .captures_iter(&input)
        .map(|caps| caps.get(0).unwrap().range())
        .collect()
}

fn multiply(input: &String, skips: &Vec<Range<usize>>) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re
        .captures_iter(&input)
        .filter(|caps| {
            let n = caps.get(0).unwrap().range();

            !skips.iter().any(|r| r.start <= n.end && n.start <= r.end)
        })
        .map(|caps| {
            let (_, [ls, rs]) = caps.extract();
            let left = ls.parse::<u32>().unwrap();
            let right = rs.parse::<u32>().unwrap();

            left * right
        })
        .sum()
}

#[test]
fn test_multiply_input() {
    let shopkeeper_data = fs::read_to_string("test_input").unwrap();

    assert_eq!(multiply(&shopkeeper_data, &vec![]), 161);
}

#[test]
fn test_multiply_input_with_skips() {
    let shopkeeper_data = fs::read_to_string("test_input2").unwrap();
    let skips = parse_skips(&shopkeeper_data);

    assert_eq!(multiply(&shopkeeper_data, &skips), 48);
}
