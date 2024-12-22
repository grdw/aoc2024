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
    let w_len = 5;
    let max_shift = 18;

    for l in fs::read_to_string(input).unwrap().lines() {
        let n = l.parse::<u64>().unwrap();
        let b = generate_bananas(n, steps);
        let mut set = HashSet::new();

        for i in 0..(b.len() - w_len) {
            let w = &b[i..i+w_len];
            let d: u32 = (0..w.len() - 1)
                .map(|j| {
                    let k = j + 1;
                    let d = ((w[k] - w[j]) + max_shift) as u32;

                    d.pow(k as u32)
                })
                .sum();

            if set.contains(&d) {
                continue
            }

            let bananas = w[w.len() - 1] as u64;
            set.insert(d);
            map
                .entry(d)
                .and_modify(|n| *n += bananas)
                .or_insert(bananas);
        }
    }

    *map.values().max().unwrap()
}

fn generate_bananas(n: u64, steps: usize) -> Vec<i8> {
    let mut m = n;

    (0..steps).map(|_| {
        let z = (m % 10) as i8;
        m = generate(m);
        z
    }).collect()
}

#[test]
fn test_generate_bananas() {
    assert_eq!(most_bananas("3", 10), 6);
    assert_eq!(most_bananas("2", 2000), 23);
}

fn generate(n: u64) -> u64 {
    let m = mix_prune(n * 64, n);
    let o = mix_prune(m / 32, m);
    mix_prune(o * 2048, o)
}

fn mix_prune(m: u64, n: u64) -> u64 {
    (m ^ n) % PRUNE
}

#[test]
fn test_generate() {
    assert_eq!(generate_rec(123, 0, 1), 15887950);
    assert_eq!(generate_rec(123, 0, 10), 5908254);
}
