use std::fs;
use std::collections::{HashMap, HashSet};

const PRUNE: u64 = 16777216;
const W_LEN: usize = 5;
const MAX_SHIFT: i16 = 18;

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

fn most_bananas(input: &'static str, steps: usize) -> i16 {
    let mut map = HashMap::new();
    let mut set = HashSet::new();

    for l in fs::read_to_string(input).unwrap().lines() {
        let mut m = l.parse::<u64>().unwrap();
        let mut w = vec![i16::MAX; W_LEN];

        for _ in 0..steps {
            let z = (m % 10) as i16;
            m = generate(m);
            w.remove(0);
            w.push(z);

            let d: u32 = (0..w.len() - 1)
                .map(|j| {
                    let k = j + 1;
                    let d = ((w[k] - w[j]) + MAX_SHIFT) as u32;

                    d.pow(k as u32)
                })
                .sum();

            if set.contains(&d) {
                continue
            }

            let bc = w[w.len() - 1];
            set.insert(d);
            map.entry(d).and_modify(|n| *n += bc).or_insert(bc);
        }

        set.clear();
    }

    *map.values().max().unwrap()
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
