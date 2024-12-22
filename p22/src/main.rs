use std::fs;
use std::collections::{HashMap, HashSet};

const PRUNE: u64 = 16777216;

fn main() {
    println!("p1 {}", generate_range("input"));
    println!("p2 {}", most_bananas("input", 2000));
}

fn generate_range(input: &'static str) -> u64 {
    fs::read_to_string(input).unwrap().lines().map(|l| {
        let n = l.parse::<u64>().unwrap();
        generate_rec(n, 0, 2000)
    }).sum()
}

#[test]
fn test_generate_range() {
    assert_eq!(generate_range("1"), 37327623);
}

fn generate_rec(n: u64, c: usize, steps: usize) -> u64 {
    if c == steps {
        return n
    }

    generate_rec(generate(n), c + 1, steps)
}

fn most_bananas(input: &'static str, steps: usize) -> u64 {
    let mut map = HashMap::new();

    for l in fs::read_to_string(input).unwrap().lines() {
        let n = l.parse::<u64>().unwrap();
        let b = generate_bananas(n, steps);
        let mut set = HashSet::new();

        for i in 0..(b.len() - 5) {
            let window = &b[i..i+5];
            let mut d = vec![];
            for j in 0..window.len() - 1  {
                d.push(window[j + 1] - window[j]);
            }

            if set.contains(&d) {
                continue
            }

            let bananas = window[window.len() - 1] as u64;
            set.insert(d.clone());
            map
                .entry(d)
                .and_modify(|n| *n += bananas)
                .or_insert(bananas);
        }
    }

    let mut max = 0;

    for value in map.values() {
        if *value > max {
            max = *value;
        }
    }

    max
}

fn generate_bananas(n: u64, steps: usize) -> Vec<i8> {
    let mut queue = vec![];
    let mut i = 0;
    let mut diffs = vec![];
    queue.push(n);

    while let Some(p) = queue.pop() {
        if i == steps {
            continue
        }

        let z = (p % 10) as i8;
        let m = generate(p);
        queue.push(m);
        diffs.push(z);
        i += 1;
    }

    diffs
}

#[test]
fn test_generate_bananas() {
    assert_eq!(most_bananas("3", 10), 6);
    assert_eq!(most_bananas("2", 2000), 23);
}

fn generate(n: u64) -> u64 {
    step3(step2(step1(n)))
}

fn step1(n: u64) -> u64 {
    let m = n * 64;
    let o = m ^ n;
    o % PRUNE
}

fn step2(n: u64) -> u64 {
    let m = n / 32;
    let o = m ^ n;
    o % PRUNE
}

fn step3(n: u64) -> u64 {
    let m = n * 2048;
    let o = m ^ n;
    o % PRUNE
}

#[test]
fn test_generate() {
    assert_eq!(generate_rec(123, 0, 1), 15887950);
    assert_eq!(generate_rec(123, 0, 10), 5908254);
}
