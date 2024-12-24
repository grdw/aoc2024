use std::fs;
use std::collections::{HashSet, HashMap};

type Nodes = Vec<String>;
type Edges = Vec<(usize, usize)>;

fn main() {
    let (nodes, edges) = parse("input");
    println!("p1 {}", t_count(&nodes, &edges));
    println!("p2 {}", max_connection_count(&nodes, &edges));
}

fn parse(input: &'static str) -> (Nodes, Edges) {
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

    let string_nodes = nodes.iter().map(|s| s.to_string()).collect();
    (string_nodes, edges)
}

fn t_count(nodes: &Nodes, edges: &Edges) -> usize {
    let mut queue = edges.clone();
    let mut set = HashSet::new();

    while let Some((l, r)) = queue.pop() {
        let f = edges
            .iter()
            .filter(|&&(vl, vr)| !(vl == r && vr == l));

        for (al, _) in f.clone().filter(|&&(_, vr)| vr == l) {
            for (_, br) in f.clone().filter(|&&(vl, _)| vl == r) {
                if al == br {
                    let mut list = vec![l, r, *al];
                    list.sort();

                    if list.iter().any(|n| nodes[*n].starts_with("t")) {
                        set.insert(list);
                    }
                    break;
                }
            }
        }
    }

    set.len()
}

#[test]
fn test_t_count() {
    let (nodes, edges) = parse("1");
    assert_eq!(t_count(&nodes, &edges), 7)
}

fn max_connection_count(nodes: &Nodes, edges: &Edges) -> String {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut b: HashMap<String, usize> = HashMap::new();
    let mut max_comb: Vec<usize> = vec![];

    for (l, r) in edges {
        graph.entry(*l).or_default().push(*r);
    }

    for i in 0..nodes.len() {
        let mut x = graph[&i].clone();
        x.push(i);
        for key in &mut subsets(&x[..], 0) {
            key.sort();

            let c = format!("{:?}", key);
            let count = *b.get(&c).unwrap_or(&0);
            b.insert(c, count + 1);

            if count + 1 == key.len() && key.len() > max_comb.len() {
                max_comb = key.clone();
            }
        }
    }

    let mut list: Vec<&str> = max_comb
        .iter()
        .map(|n| nodes[*n].as_str())
        .collect();

    list.sort();
    list.join(",")
}

fn subsets(arr: &[usize], i: usize) -> Vec<Vec<usize>> {
    if i == arr.len() {
        vec![vec![]]
    } else {
        let rest = subsets(arr, i + 1);
        rest.iter()
            .flat_map(|x| {
                vec![x.clone(), {
                    let mut subset = vec![arr[i]];
                    subset.extend_from_slice(x);
                    subset
                }]
            })
            .collect()
    }
}

#[test]
fn test_max_connection_count() {
    let (nodes, edges) = parse("1");
    assert_eq!(
        max_connection_count(&nodes, &edges),
        String::from("co,de,ka,ta")
    )
}
