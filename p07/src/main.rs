use std::fs;
use std::collections::HashMap;

type TestValues = HashMap<u64, Vec<u64>>;

fn main() {
    let test_values = parse("input");

    println!("p1 {}", correct_test_values(&test_values, false));
    println!("p2 {}", correct_test_values(&test_values, true));
}

fn parse(input: &'static str) -> TestValues {
    let mut map = HashMap::new();

    for line in fs::read_to_string(input).unwrap().lines() {
        let (total_s, nums_s) = line.split_once(": ").unwrap();
        let total = total_s.parse::<u64>().unwrap();
        let nums = nums_s
            .split(" ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();

        map.insert(total, nums);
    }

    map
}

fn correct_test_values(test_values: &TestValues, conc: bool) -> u64 {
    test_values
        .iter()
        .filter(|&(total, nums)| is_corr(nums[0], 1, *total, nums, conc))
        .map(|(total, _)| total)
        .sum()
}

fn is_corr(k: u64, i: usize, l: u64, n: &Vec<u64>, conc: bool) -> bool {
    if i == n.len() {
        return k == l
    }

    is_corr(k + n[i], i + 1, l, n, conc) ||
    is_corr(k * n[i], i + 1, l, n, conc) ||
    (conc && is_corr(concat(k, n[i]), i + 1, l, n, conc))
}

#[test]
fn test_correct_test_values() {
    let test_values = parse("1");

    assert_eq!(correct_test_values(&test_values, false), 3749)
}

fn concat(n: u64, m: u64) -> u64 {
    let m_len = ((m as f64).log10().floor() + 1.0) as u32;
    let k = 10_u64.pow(m_len);

    (n * k) + m
}

#[test]
fn test_concat() {
    assert_eq!(concat(123, 45), 12345);
}

#[test]
fn test_concat_correct() {
    let test_values = parse("1");

    assert_eq!(correct_test_values(&test_values, true), 11387)
}
