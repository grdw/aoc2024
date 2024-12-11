use std::fs;
use std::collections::HashMap;

type Stones = Vec<u64>;

fn main() {
    let stones = parse("input");
    let stone_count = count_stones(&stones, 25);
    println!("p1 {}", stone_count);
    let stone_count = count_stones(&stones, 75);
    println!("p2 {}", stone_count);
}

fn parse(input: &'static str) -> Stones {
    fs::read_to_string(input)
        .unwrap()
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn count_stones(stones: &Stones, blinks: u16) -> usize {
    let mut map:HashMap<u64, usize> = HashMap::new();

    for stone in stones {
        map.insert(*stone, 1);
    }

    for _ in 0..blinks {
        let mut cache: HashMap<u64, usize> = HashMap::new();

        for (&stone, &count) in map.iter() {
            if stone == 0 {
                *cache.entry(1).or_default() += count;
            } else {
                let dl = digit_length(stone);
                if dl % 2 == 0 {
                    let k = 10_u64.pow(dl / 2);
                    let l = stone / k;
                    let r = stone - (l * k);

                    *cache.entry(l).or_default() += count;
                    *cache.entry(r).or_default() += count;
                } else {
                    *cache.entry(stone * 2024).or_default() += count;
                }
            }
        }
        map = cache;
    }

    map.values().sum()
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
