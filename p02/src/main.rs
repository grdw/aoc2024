use std::fs;

fn main() {
    let l = parse("input");

    println!("p1 {}", safety_clearance(&l));
    println!("p2 {}", safety_clearance_dampened(&l));
}

fn parse(input: &'static str) -> Vec<Vec<u32>> {
    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            line
                .split(" ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        }).collect()
}

#[test]
fn test_safety_clearance() {
    let l = parse("test_input");
    assert_eq!(safety_clearance(&l), 2)
}

fn safety_clearance(l: &Vec<Vec<u32>>) -> usize {
    l.iter().filter(|n| is_safe(*n)).count()
}

#[test]
fn test_safety_clearance_dampened() {
    let l = parse("test_input");
    assert_eq!(safety_clearance_dampened(&l), 4)
}

fn safety_clearance_dampened(l: &Vec<Vec<u32>>) -> usize {
    l.iter().filter(|n| is_safe(*n) || is_safe_minus_one(*n)).count()
}

// This is not the most memory efficient way of doing this, but
// considering how fast this still is... I don't really feel
// the need to optimize it all that much.
fn is_safe_minus_one(report: &Vec<u32>) -> bool {
    (0..report.len()).any(|j| {
        let mut q = report.clone();
        q.remove(j);
        is_safe(&q)
    })
}

fn is_safe(report: &Vec<u32>) -> bool {
    let end = report.len() - 1;
    let all_increasing = (0..end).all(|i| {
        let l = report[i];
        let r = report[i + 1];
        l > r && (l - r) <= 3
    });

    let all_decreasing = (0..end).all(|i| {
        let l = report[i];
        let r = report[i + 1];
        l < r && (r - l) <= 3
    });

    return all_increasing || all_decreasing
}

