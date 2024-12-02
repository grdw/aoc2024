use std::fs;

fn main() {
    let (mut l, mut m) = parse("input");
    println!("p1 {}", total_distance(&mut l, &mut m));
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
