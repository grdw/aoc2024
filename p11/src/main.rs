use std::fs;
use std::collections::VecDeque;

type Stones = Vec<u64>;

fn main() {
    let stones = parse("input");
    let stone_count = count_stones(&stones, 25);
    println!("p1 {}", stone_count);
}

fn parse(input: &'static str) -> Stones {
    fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn count_stones(stones: &Stones, blinks: u8) -> usize {
    stones
        .iter()
        .map(|stone| count_stone_blink(*stone, blinks))
        .sum()
}

fn count_stone_blink(stone: u64, blinks: u8) -> usize {
    let mut deq = VecDeque::new();
    let mut count = 0;
    deq.push_back((stone, 0));

    while let Some((stone, blink)) = deq.pop_front() {
        if blink == blinks {
            count += 1;
            continue
        }

        let l = digit_length(stone);
        if stone == 0 {
            deq.push_back((1, blink + 1));
        } else if l % 2 == 0 {
            let k = 10_u64.pow(l / 2);
            let l = stone / k;
            let r = stone - (l * k);

            deq.push_back((l, blink + 1));
            deq.push_back((r, blink + 1));
        } else {
            deq.push_back((stone * 2024, blink + 1));
        }
    }

    count
}

fn digit_length(stone: u64) -> u32 {
    ((stone as f64).log10().floor() + 1.0) as u32
}

#[test]
fn test_count_stones() {
    let stones = parse("1");
    assert_eq!(count_stones(&stones, 1), 3);
    assert_eq!(count_stones(&stones, 2), 4);
    assert_eq!(count_stones(&stones, 6), 22);
    assert_eq!(count_stones(&stones, 25), 55312);
}
