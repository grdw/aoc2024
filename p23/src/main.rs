use std::fs;
use std::collections::HashSet;

fn main() {
    println!("p1 {}", t_count("input"));
}

fn t_count(input: &'static str) -> usize {
    let mut nodes = vec![];
    let mut edges = vec![];
    let input = fs::read_to_string(input).unwrap();
    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();

        if !nodes.contains(&left) {
            nodes.push(left);
        }
        if !nodes.contains(&right) {
            nodes.push(right);
        }

        let n = nodes.iter().position(|&l| l == left).unwrap();
        let m = nodes.iter().position(|&r| r == right).unwrap();
        edges.push((n, m));
        edges.push((m, n));
    }

    let mut queue = edges.clone();
    let mut set = HashSet::new();

    while let Some((l, r)) = queue.pop() {
        let r_neighbors: Vec<&(usize, usize)> = edges
            .iter()
            .filter(|&&(vl, _)| vl == r)
            .filter(|&&(vl, vr)| !(vl == r && vr == l))
            .collect();

        let l_neighbors: Vec<&(usize, usize)> = edges
            .iter()
            .filter(|&&(_, vr)| vr == l)
            .filter(|&&(vl, vr)| !(vl == r && vr == l))
            .collect();

        for (al, _) in &l_neighbors {
            for (_, br) in &r_neighbors {
                if al == br {
                    let mut list = vec![l, r, *al];
                    list.sort();

                    if list.iter().any(|n| nodes[*n].starts_with("t")) {
                        set.insert(list);
                    }
                }
            }
        }
    }

    set.len()
}

#[test]
fn test_t_count() {
    assert_eq!(t_count("1"), 7)
}
