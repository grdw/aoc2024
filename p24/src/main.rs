use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap, HashSet};

type RNode = Rc<RefCell<Node>>;

#[derive(Debug)]
enum Instruction {
    And,
    Xor,
    Or
}

#[derive(Debug)]
struct Node {
    name: String,
    instruction: Option<Instruction>,
    value: Option<u8>,
    left: Option<RNode>,
    right: Option<RNode>
}

impl Node {
    fn new(name: String, instruction: Instruction, left: RNode, right: RNode) -> RNode {
        Rc::new(
            RefCell::new(
                Node {
                    name,
                    value: None,
                    instruction: Some(instruction),
                    left: Some(left),
                    right: Some(right),
                }
            )
        )
    }

    fn child(name: String) -> RNode {
        Rc::new(
            RefCell::new(
                Node {
                    name,
                    instruction: None,
                    left: None,
                    right: None,
                    value: None
                }
            )
        )
    }
}

struct Tree {
    nodes: Vec<RNode>
}

fn main() {
    let tree = parse("input");
    println!("p1 {}", decimal_number(&tree));
    //println!("p2 {}", list_swaps(nodes.clone(), &mut edges, 4));
}

fn parse(input: &'static str) -> Tree {
    let mut tree = Tree { nodes: vec![] };
    let input = fs::read_to_string(input).unwrap();
    let (input_wires, wires) = input.split_once("\n\n").unwrap();

    let mut input_map = HashMap::new();
    for input_wire in input_wires.lines() {
        let (node, value) = input_wire.split_once(": ").unwrap();
        let v = value.parse::<u8>().unwrap();
        input_map.insert(node.to_string(), v);
    }

    let mut wire_map = HashMap::new();
    for wire in wires.lines() {
        let (inputs, output) = wire.split_once(" -> ").unwrap();
        let operations: Vec<&str> = inputs.split(" ").collect();
        let instruction = match operations[1] {
            "XOR" => Instruction::Xor,
            "OR"  => Instruction::Or,
            "AND" => Instruction::And,
            _ => panic!("Bad input")
        };

        let left = Node::child(operations[0].to_string());
        let right = Node::child(operations[2].to_string());
        wire_map.insert(
            output.to_string(),
            Node::new(
                output.to_string(),
                instruction,
                left,
                right,
            )
        );
    }

    build_tree(&mut tree, &mut wire_map, input_map);
    tree
}

fn build_tree(
    tree: &mut Tree,
    map: &mut HashMap<String, RNode>,
    input_map: HashMap<String, u8>) {

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    for node in map.values() {
        queue.push_front(node.clone());
    }

    while let Some(n) = queue.pop_front() {
        let mut node = n.borrow_mut();

        if !visited.contains(&node.name) {
            if node.name.starts_with("z") {
                tree.nodes.push(n.clone());
            }
        }

        if let Some(value) = input_map.get(&node.name) {
            node.value = Some(*value);
        }

        match (node.left.clone(), node.right.clone()) {
            (Some(l), Some(r)) => {
                queue.push_back(l);
                queue.push_back(r);
            },
            _ => {
                if let Some(x) = map.get(&node.name) {
                    let n = x.borrow();
                    node.left = n.left.clone();
                    node.right = n.right.clone();
                }
            }
        }
        visited.insert(node.name.clone());
    }
}

fn decimal_number(tree: &Tree) -> usize {
    //let mut queue = VecDeque::new();
    //for node in &tree.nodes {
    //    println!(" ============ ");
    //    println!("{:?}", node);
    //    queue.push_back(node.clone());
    //}

    //while let Some(v) = queue.pop_front() {
    //    let mut node = v.borrow_mut();
    //    println!("{:?}", node.name);

    //    if let (Some(l), Some(r)) = (node.left.clone(), node.right.clone()) {
    //        let left = l.borrow();
    //        let right = r.borrow();
    //        match (left.value, right.value) {
    //            (Some(x), Some(y)) => {
    //                if let Some(inst) = &node.instruction {
    //                    let value = match inst {
    //                        Instruction::And => x & y,
    //                        Instruction::Or => x | y,
    //                        Instruction::Xor => x ^ y
    //                    };

    //                    println!("Set {} to {}", value, node.name);
    //                    node.value = Some(value);
    //                }
    //            },
    //            _ => {
    //                println!("XYZ");
    //                queue.push_front(l.clone());
    //                queue.push_front(r.clone());
    //            }
    //        }
    //    }
    //}

    0
}

#[test]
fn test_decimal_number() {
    let tree = parse("2");
    assert_eq!(decimal_number(&tree), 2024);
    let tree = parse("1");
    assert_eq!(decimal_number(&tree), 4);
}

//fn form_digit_from(nodes: &Nodes, search: &'static str) -> Option<usize> {
//    let mut digits = vec![];
//    let mut total = String::new();
//
//    for n in nodes {
//        if let Node::Wire(name, value) = n {
//            if name.starts_with(search) {
//               digits.push((name, value));
//            }
//        }
//    }
//    digits.sort();
//
//    for (_, v) in digits.iter().rev() {
//        match v {
//            Some(w) => {
//                let q = char::from_digit(*w as u32, 10).unwrap();
//                total.push(q);
//            },
//            None => {
//                return None
//            }
//        }
//    }
//
//    Some(usize::from_str_radix(&total, 2).unwrap())
//}
//
//fn resolve(nodes: &mut Nodes, edges: &Edges) {
//    let mut queue = VecDeque::new();
//    for (w1, op, w2, out) in edges {
//        match (&nodes[*w1], &nodes[*w2]) {
//            (Node::Wire(_, v1), Node::Wire(_, v2)) => {
//                if v1.is_some() && v2.is_some() {
//                    queue.push_back((*w1, *op, *w2, *out));
//                }
//            },
//            _ => panic!("Something is fucked")
//        }
//    }
//
//    while let Some((w1, op, w2, out)) = queue.pop_front() {
//        match (&nodes[w1], &nodes[op], &nodes[w2]) {
//            (Node::Wire(_, v1), Node::Op(x), Node::Wire(_, v2)) => {
//                if let (Some(a), Some(b)) = (v1, v2) {
//                    let result = match x.as_str() {
//                        "AND" => *a & *b,
//                        "OR" => *a | *b,
//                        "XOR" => *a ^ *b,
//                        _ => panic!("Invalid x")
//                    };
//
//                    if let Node::Wire(_, ref mut value) = &mut nodes[out] {
//                        *value = Some(result);
//
//                        for edge in edges {
//                            if edge.0 == out || edge.2 == out {
//                                queue.push_back(*edge);
//                            }
//                        }
//                    }
//                }
//            },
//            _ => continue
//        }
//    }
//}
//
//fn list_swaps(nodes: Nodes, edges: &mut Edges, max_swaps: usize) -> String {
//    fn backtrack(
//        y: usize,
//        x: usize,
//        nodes: Nodes,
//        edges: &mut Edges,
//        swaps: &mut Vec<(usize, usize)>,
//        set: &mut HashSet<usize>,
//        start: usize,
//        max_swaps: usize,
//    ) -> Option<Vec<(usize, usize)>> {
//        if swaps.len() == max_swaps {
//            let mut nodes = nodes.clone();
//
//            for (i, j) in swaps.iter() {
//                let a = edges[*i].3;
//                let b = edges[*j].3;
//                edges[*i].3 = b;
//                edges[*j].3 = a;
//            }
//
//            resolve(&mut nodes, &edges);
//
//            if let Some(z) = form_digit_from(&nodes, "z") {
//                println!("{} + {} = {}", y, x, z);
//                if y + x == z {
//                    println!("{:?}", swaps);
//                    println!("FOUND IT");
//                    return Some(swaps.clone());
//                }
//
//                // Swap everything back
//                for (j, i) in swaps.iter() {
//                    let a = edges[*i].3;
//                    let b = edges[*j].3;
//                    edges[*i].3 = b;
//                    edges[*j].3 = a;
//                }
//            }
//
//            return None;
//        }
//
//        for i in start..edges.len() {
//            for j in (i + 1)..edges.len() {
//                let a = &edges[i];
//                let b = &edges[j];
//
//                if a.3 == b.0 || a.3 == b.1 {
//                    continue
//                }
//
//                if b.3 == a.0 || b.3 == a.1 {
//                    continue
//                }
//
//                if !set.contains(&i) && !set.contains(&j) {
//                    set.insert(i);
//                    set.insert(j);
//                    // Check if this pair (i, i+1) is valid
//                    swaps.push((i, j));
//
//                    // Recur for the next set of swaps
//                    if let Some(final_swaps) = backtrack(
//                        y,
//                        x,
//                        nodes.clone(),
//                        edges,
//                        swaps,
//                        set,
//                        i + 1,
//                        max_swaps
//                    ) {
//                        return Some(final_swaps);
//                    }
//
//                    // Backtrack
//                    swaps.pop();
//                    set.remove(&i);
//                    set.remove(&j);
//                }
//            }
//        }
//
//        None
//    }
//
//    let mut set = HashSet::new();
//    let mut temp_swaps = vec![];
//    let y = form_digit_from(&nodes, "y").unwrap();
//    let x = form_digit_from(&nodes, "x").unwrap();
//    let swaps = backtrack(
//        y,
//        x,
//        nodes.clone(),
//        edges,
//        &mut temp_swaps,
//        &mut set,
//        0,
//        max_swaps
//    ).unwrap();
//
//    let mut x: Vec<&str> = vec![];
//    for (a, b) in &swaps {
//        match (&nodes[edges[*a].3], &nodes[edges[*b].3]) {
//            (Node::Wire(name, _), Node::Wire(name_b, _)) => {
//                x.push(name.as_str());
//                x.push(name_b.as_str());
//            },
//            _ => panic!("You swapped garbage my boy")
//        }
//    }
//
//    x.sort();
//    x.join(",")
//}

//#[test]
//fn test_list_swaps() {
//    let (nodes, mut edges) = parse("3");
//    assert_eq!(
//        list_swaps(nodes, &mut edges, 2),
//        String::from("z01,z02,z04,z05")
//    );
//}
