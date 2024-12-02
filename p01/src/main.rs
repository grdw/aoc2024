use std::fs;
use std::collections::HashMap;

fn main() {
    let (mut l, mut m) = parse("input");
    println!("p1 {}", total_distance(&mut l, &mut m));
    println!("p2 {}", similarity_score(&l, &m));
}

fn parse(input: &'static str) -> (Vec<u32>, Vec<u32>) {
    let mut k = vec![];
    let mut l = vec![];

    for line in fs::read_to_string(input).unwrap().lines() {
        let (ns, ms) = line.split_once("   ").unwrap();
        let n = ns.parse::<u32>().unwrap();
        let m = ms.parse::<u32>().unwrap();
        k.push(n);
        l.push(m);
    }

    (k, l)
}

fn total_distance(l: &mut Vec<u32>, m: &mut Vec<u32>) -> u32 {
    l.sort();
    m.sort();

    let mut d = 0;
    for i in 0..l.len() {
        d += l[i].abs_diff(m[i])
    }
    return d
}

#[test]
fn test_distance() {
    let (mut v1, mut v2) = parse("test_input");
    assert_eq!(total_distance(&mut v1, &mut v2), 11)
}

fn similarity_score(l: &Vec<u32>, m: &Vec<u32>) -> u32 {
    let mut h: HashMap<u32, u32> = HashMap::new();
    let mut s = 0;
    for i in m.iter() {
        h.entry(*i).and_modify(|c| *c += 1).or_insert(1);
    }

    for j in l.iter() {
        s += j * h.get(j).unwrap_or(&0)
    }
    s
}

#[test]
fn test_similarity_score() {
    let (v1, v2) = parse("test_input");
    assert_eq!(similarity_score(&v1, &v2), 31)
}
