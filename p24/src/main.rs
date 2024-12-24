use std::fs;
use std::collections::{VecDeque, HashMap};

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Wire(String, Option<bool>),
    Op(String)
}
type Nodes = Vec<Node>;
type Edge = (usize, usize, usize, usize);
type Edges = Vec<Edge>;

fn main() {
    let (mut nodes, edges) = parse("input");
    println!("p1 {}", decimal_number(&mut nodes, &edges));
}

fn parse(input: &'static str) -> (Nodes, Edges) {
    let mut nodes = vec![];
    let mut edges = vec![];

    let input = fs::read_to_string(input).unwrap();
    let (input_wires, wires) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    for input_wire in input_wires.lines() {
        let (node, value) = input_wire.split_once(": ").unwrap();
        let v = value == "1";
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

fn decimal_number(nodes: &mut Nodes, edges: &Edges) -> usize {
    let mut queue = VecDeque::new();
    for (w1, op, w2, out) in edges {
        match (&nodes[*w1], &nodes[*w2]) {
            (Node::Wire(_, _), Node::Wire(_, _)) => {
                queue.push_back((w1, op, w2, out));
            }
            _ => continue
        }
    }

    while let Some((w1, op, w2, out)) = queue.pop_front() {
        match (&nodes[*w1], &nodes[*op], &nodes[*w2]) {
            (Node::Wire(_, v1), Node::Op(x), Node::Wire(_, v2)) => {
                match (v1, v2) {
                    (Some(a), Some(b)) => {
                        let result = match x.as_str() {
                            "AND" => *a && *b,
                            "OR" => *a || *b,
                            "XOR" => *a ^ *b,
                            _ => panic!("Invalid x")
                        };

                        if let Node::Wire(_, ref mut value) = &mut nodes[*out] {
                            *value = Some(result);
                        }

                    },
                    // Not sure if this is going to do shit
                    _ => queue.push_back((w1, op, w2, out))
                }
            },
            _ => continue
        }
    }

    let mut digits = vec![];
    for n in nodes {
        if let Node::Wire(name, value) = n {
            if name.starts_with("z") {
               digits.push((name, value));
            }
        }
    }
    digits.sort();

    let mut total = String::new();
    for (_, v) in digits.iter().rev() {
        let q = match v {
            Some(true) => '1',
            Some(false) => '0',
            _ => panic!("Error")
        };
        total.push(q);
    }

    usize::from_str_radix(&total, 2).unwrap()
}

#[test]
fn test_decimal_number() {
    let (mut nodes, edges) = parse("1");
    assert_eq!(decimal_number(&mut nodes, &edges), 4);

    let (mut nodes, edges) = parse("2");
    assert_eq!(decimal_number(&mut nodes, &edges), 2024);
}
