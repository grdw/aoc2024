use std::fs;
use std::collections::HashMap;

type TestValues = HashMap<u64, Vec<u64>>;

fn main() {
    let test_values = parse("input");

    println!("p1 {}", correct_test_values(&test_values));
}

fn parse(input: &'static str) -> TestValues {
    let mut map = HashMap::new();

    for line in fs::read_to_string(input).unwrap().lines() {
        let (total_s, numbers_s) = line.split_once(": ").unwrap();
        let total = total_s.parse::<u64>().unwrap();
        let numbers = numbers_s
            .split(" ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();

        map.insert(total, numbers);
    }

    map
}

fn correct_test_values(test_values: &TestValues) -> u64 {
    let mut t = 0;

    for (total, numbers) in test_values {
        if correct(numbers[0], *total, 1, numbers) {
            t += total
        }
    }

    t
}

fn correct(it: u64, total: u64, i: usize, numbers: &Vec<u64>) -> bool {
    if i == numbers.len() {
        return it == total
    }

    correct(it + numbers[i], total, i + 1, numbers) ||
    correct(it * numbers[i], total, i + 1, numbers)
}

#[test]
fn test_correct_test_values() {
    let test_values = parse("1");
    assert_eq!(correct_test_values(&test_values), 3749)
}
