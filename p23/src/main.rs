use std::fs;
use std::collections::{VecDeque, HashSet, HashMap};

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
    let mut visited = HashSet::new();
    let mut max_comb = vec![];

    for (l, r) in edges {
        graph.entry(*l).or_default().push(*r);
    }

    for &i in graph.keys() {
        let mut queue = VecDeque::new();
        let mut comb = vec![];
        queue.push_back(i);
        visited.insert(i);

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.get(&current) {
                comb.push(current);
                for &neighbor in neighbors {
                    if visited.contains(&neighbor) {
                        continue
                    }

                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        if comb.len() > max_comb.len() {
            max_comb = comb;
        }
    }

    //let mut set = HashSet::new();
    for c in &max_comb {
        println!("{:?}", graph[c].len());
    }

    //let mut sorted: Vec<&str> = set
    //    .iter()
    //    .map(|s| nodes[**s].as_str())
    //    .collect();

    //sorted.sort();
    //sorted.join(",")
    String::from("WRONG")
}



#[test]
fn test_max_connection_count() {
    let (nodes, edges) = parse("1");
    assert_eq!(
        max_connection_count(&nodes, &edges),
        String::from("co,de,ka,ta")
    )
}