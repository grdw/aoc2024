use std::fs;
use std::io;
use regex::Regex;

struct Instruction(u32, u32);

fn main() {
    let instructions = parse("input").unwrap();
    println!("p1 {}", multiply(&instructions));
    let instructions = parse_with_skips("input").unwrap();
    println!("p2 {}", multiply(&instructions));
}

fn multiply(instructions: &Vec<Instruction>) -> u32 {
    instructions.iter().map(|i| i.0 * i.1).sum()
}

fn parse(input: &'static str) -> io::Result<Vec<Instruction>> {
    let shopkeeper_data = fs::read_to_string(input)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Ok(
        re
            .captures_iter(&shopkeeper_data)
            .map(|caps| {
                let (_, [ls, rs]) = caps.extract();
                let left = ls.parse::<u32>().unwrap();
                let right = rs.parse::<u32>().unwrap();

                Instruction(left, right)
            })
            .collect()
    )
}

#[test]
fn test_mul_input() {
    let t = parse("test_input").unwrap();
    assert_eq!(multiply(&t), 161)
}

fn parse_with_skips(input: &'static str) -> io::Result<Vec<Instruction>> {
    let shopkeeper_data = fs::read_to_string(input)?;
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut enabled = true;
    let mut instructions = vec![];

    for caps in re.captures_iter(&shopkeeper_data) {
        let s = caps.get(1);
        let l = caps.get(2);
        let r = caps.get(3);

        if l.is_none() {
            let signal = s.unwrap().as_str();
            enabled = signal == "do()";
        } else if enabled {
            let left = l.unwrap().as_str().parse::<u32>().unwrap();
            let right = r.unwrap().as_str().parse::<u32>().unwrap();
            instructions.push(Instruction(left, right));
        }
    }

    Ok(instructions)
}

#[test]
fn test_mul_input_with_skips() {
    let t = parse_with_skips("test_input2").unwrap();
    assert_eq!(multiply(&t), 48)
}
