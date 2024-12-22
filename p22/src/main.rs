use std::fs;

const PRUNE: u128 = 16777216;

fn main() {
    println!("p1 {}", generate_range("input"));
}

fn generate_range(input: &'static str) -> u128 {
    fs::read_to_string(input).unwrap().lines().map(|l| {
        let n = l.parse::<u128>().unwrap();
        generate_rec(n, 0, 2000)
    }).sum()
}

#[test]
fn test_generate_range() {
    assert_eq!(generate_range("1"), 37327623);
}

fn generate_rec(n: u128, c: usize, steps: usize) -> u128 {
    if c == steps {
        return n
    }

    generate_rec(generate(n), c + 1, steps)
}

fn generate(n: u128) -> u128 {
    step3(step2(step1(n)))
}

fn step1(n: u128) -> u128 {
    let m = n * 64;
    let o = m ^ n;
    o % PRUNE
}

fn step2(n: u128) -> u128 {
    let m = n / 32;
    let o = m ^ n;
    o % PRUNE
}

fn step3(n: u128) -> u128 {
    let m = n * 2048;
    let o = m ^ n;
    o % PRUNE
}

#[test]
fn test_generate() {
    assert_eq!(generate_rec(123, 0, 1), 15887950);
    assert_eq!(generate_rec(123, 0, 10), 5908254);
}
