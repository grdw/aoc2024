use std::fs;
use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Wire(String, Option<u8>),
    Op(String)
}
type Nodes = Vec<Node>;
type Edge = (usize, usize, usize, usize);
type Edges = Vec<Edge>;

fn main() {
    let (nodes, mut edges) = parse("input");
    println!("p1 {}", decimal_number(nodes.clone(), &edges));
    println!("p2 {}", list_swaps(nodes.clone(), &mut edges, 4));
}

fn parse(input: &'static str) -> (Nodes, Edges) {
    let mut nodes = vec![];
    let mut edges = vec![];

    let input = fs::read_to_string(input).unwrap();
    let (input_wires, wires) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    for input_wire in input_wires.lines() {
        let (node, value) = input_wire.split_once(": ").unwrap();
        let v = value.parse::<u8>().unwrap();
        map.insert(node, v);
    }

    for wire in wires.lines() {
        let (inputs, output) = wire.split_once(" -> ").unwrap();
        let out = Node::Wire(output.to_string(), None);
        let mut temp_nodes = vec![
            out
        ];

        for t in inputs.split(" ") {
            let q = t.to_string();
            let n = match t {
                "AND" | "OR" | "XOR" => Node::Op(q),
                _ => {
                    match map.get(t) {
                        Some(v) => Node::Wire(q, Some(*v)),
                        None => Node::Wire(q, None),
                    }
                }
            };
            temp_nodes.push(n);
        }

        let mut indices = vec![];
        for t in temp_nodes {
            if !nodes.contains(&t) {
                nodes.push(t.clone());
            }

            let p = nodes.iter().position(|n| *n == t).unwrap();
            indices.push(p);
        }

        edges.push((indices[1], indices[2], indices[3], indices[0]));
    }

    (nodes, edges)
}

fn decimal_number(mut nodes: Nodes, edges: &Edges) -> usize {
    resolve(&mut nodes, edges);
    form_digit_from(&nodes, "z").unwrap()
}

#[test]
fn test_decimal_number() {
    let (nodes, edges) = parse("1");
    assert_eq!(decimal_number(nodes, &edges), 4);

    let (nodes, edges) = parse("2");
    assert_eq!(decimal_number(nodes, &edges), 2024);
}

fn form_digit_from(nodes: &Nodes, search: &'static str) -> Option<usize> {
    let mut digits = vec![];
    let mut total = String::new();

    for n in nodes {
        if let Node::Wire(name, value) = n {
            if name.starts_with(search) {
               digits.push((name, value));
            }
        }
    }
    digits.sort();

    for (_, v) in digits.iter().rev() {
        match v {
            Some(w) => {
                let q = char::from_digit(*w as u32, 10).unwrap();
                total.push(q);
            },
            None => {
                return None
            }
        }
    }

    Some(usize::from_str_radix(&total, 2).unwrap())
}

fn resolve(nodes: &mut Nodes, edges: &Edges) {
    let mut queue = VecDeque::new();
    for (w1, op, w2, out) in edges {
        match (&nodes[*w1], &nodes[*w2]) {
            (Node::Wire(_, v1), Node::Wire(_, v2)) => {
                if v1.is_some() && v2.is_some() {
                    queue.push_back((*w1, *op, *w2, *out));
                }
            },
            _ => panic!("Something is fucked")
        }
    }

    while let Some((w1, op, w2, out)) = queue.pop_front() {
        match (&nodes[w1], &nodes[op], &nodes[w2]) {
            (Node::Wire(_, v1), Node::Op(x), Node::Wire(_, v2)) => {
                if let (Some(a), Some(b)) = (v1, v2) {
                    let result = match x.as_str() {
                        "AND" => *a & *b,
                        "OR" => *a | *b,
                        "XOR" => *a ^ *b,
                        _ => panic!("Invalid x")
                    };

                    if let Node::Wire(_, ref mut value) = &mut nodes[out] {
                        *value = Some(result);

                        for edge in edges {
                            if edge.0 == out || edge.2 == out {
                                queue.push_back(*edge);
                            }
                        }
                    }
                }
            },
            _ => continue
        }
    }
}

fn list_swaps(nodes: Nodes, edges: &mut Edges, max_swaps: usize) -> String {
    fn backtrack(
        y: usize,
        x: usize,
        nodes: Nodes,
        edges: &mut Edges,
        swaps: &mut Vec<(usize, usize)>,
        set: &mut HashSet<usize>,
        start: usize,
        max_swaps: usize,
    ) -> Option<Vec<(usize, usize)>> {
        if swaps.len() == max_swaps {
            let mut nodes = nodes.clone();

            for (i, j) in swaps.iter() {
                let a = edges[*i].3;
                let b = edges[*j].3;
                edges[*i].3 = b;
                edges[*j].3 = a;
            }

            resolve(&mut nodes, &edges);

            if let Some(z) = form_digit_from(&nodes, "z") {
                println!("{} + {} = {}", y, x, z);
                if y + x == z {
                    println!("{:?}", swaps);
                    println!("FOUND IT");
                    return Some(swaps.clone());
                }

                // Swap everything back
                for (j, i) in swaps.iter() {
                    let a = edges[*i].3;
                    let b = edges[*j].3;
                    edges[*i].3 = b;
                    edges[*j].3 = a;
                }
            }

            return None;
        }

        for i in start..edges.len() {
            for j in (i + 1)..edges.len() {
                let a = &edges[i];
                let b = &edges[j];

                if a.3 == b.0 || a.3 == b.1 {
                    continue
                }

                if b.3 == a.0 || b.3 == a.1 {
                    continue
                }

                if !set.contains(&i) && !set.contains(&j) {
                    set.insert(i);
                    set.insert(j);
                    // Check if this pair (i, i+1) is valid
                    swaps.push((i, j));

                    // Recur for the next set of swaps
                    if let Some(final_swaps) = backtrack(
                        y,
                        x,
                        nodes.clone(),
                        edges,
                        swaps,
                        set,
                        i + 1,
                        max_swaps
                    ) {
                        return Some(final_swaps);
                    }

                    // Backtrack
                    swaps.pop();
                    set.remove(&i);
                    set.remove(&j);
                }
            }
        }

        None
    }

    let mut set = HashSet::new();
    let mut temp_swaps = vec![];
    let y = form_digit_from(&nodes, "y").unwrap();
    let x = form_digit_from(&nodes, "x").unwrap();
    let swaps = backtrack(
        y,
        x,
        nodes.clone(),
        edges,
        &mut temp_swaps,
        &mut set,
        0,
        max_swaps
    ).unwrap();

    let mut x: Vec<&str> = vec![];
    for (a, b) in &swaps {
        match (&nodes[edges[*a].3], &nodes[edges[*b].3]) {
            (Node::Wire(name, _), Node::Wire(name_b, _)) => {
                x.push(name.as_str());
                x.push(name_b.as_str());
            },
            _ => panic!("You swapped garbage my boy")
        }
    }

    x.sort();
    x.join(",")
}

#[test]
fn test_list_swaps() {
    let (nodes, mut edges) = parse("3");
    assert_eq!(
        list_swaps(nodes, &mut edges, 2),
        String::from("z01,z02,z04,z05")
    );
}
