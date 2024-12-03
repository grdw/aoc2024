use std::fs;
use std::ops::Range;
use regex::Regex;

struct Instruction(u32, u32);

fn main() {
    let shopkeeper_data = fs::read_to_string("input").unwrap();
    let skips = parse_skips(&shopkeeper_data);
    let instructions = parse(&shopkeeper_data, &vec![]);
    println!("p1 {}", multiply(&instructions));

    let instructions = parse(&shopkeeper_data, &skips);
    println!("p2 {}", multiply(&instructions));
}

fn multiply(instructions: &Vec<Instruction>) -> u32 {
    instructions.iter().map(|i| i.0 * i.1).sum()
}

#[test]
fn test_mul_input() {
    let shopkeeper_data = fs::read_to_string("test_input").unwrap();
    let t = parse(&shopkeeper_data, &vec![]);

    assert_eq!(multiply(&t), 161)
}

fn parse_skips(input: &String) -> Vec<Range<usize>> {
    let skips_re = Regex::new(r"don't\(\).*do\(\)").unwrap();
    skips_re
        .captures_iter(&input)
        .map(|caps| {
            let m = caps.get(0).unwrap();
            m.start()..m.end()
        })
        .collect()
}

fn parse(input: &String, skips: &Vec<Range<usize>>) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re
        .captures_iter(&input)
        .filter(|caps| {
            let m = caps.get(0).unwrap();
            let n = m.start()..m.end();

            !skips.iter().any(|r| r.start <= n.end && n.start <= r.end)
        })
        .map(|caps| {
            let (_, [ls, rs]) = caps.extract();
            let left = ls.parse::<u32>().unwrap();
            let right = rs.parse::<u32>().unwrap();

            Instruction(left, right)
        })
        .collect()
}

#[test]
fn test_mul_input_with_skips() {
    let shopkeeper_data = fs::read_to_string("test_input2").unwrap();
    let skips = parse_skips(&shopkeeper_data);
    let t = parse(&shopkeeper_data, &skips);

    assert_eq!(multiply(&t), 48)
}
