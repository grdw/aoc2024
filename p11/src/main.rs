use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

type TreeNode = Rc<RefCell<Node>>;
type Stones = Vec<u64>;

#[derive(Debug)]
struct Node {
    value: u64,
    left: Option<TreeNode>,
    right: Option<TreeNode>,
}

impl Node {
    fn new(value: u64) -> TreeNode {
        Rc::new(
            RefCell::new(
                Node { value, left: None, right: None }
            )
        )
    }

    fn add_left_child(&mut self, value: u64) -> TreeNode {
        let left = Node::new(value);
        self.left = Some(left.clone());
        left
    }

    fn add_right_child(&mut self, value: u64) -> TreeNode {
        let right = Node::new(value);
        self.right = Some(right.clone());
        right
    }

    fn find_by_value(t: TreeNode, value: u64) -> Option<TreeNode> {
        let mut node = t.borrow();

        if node.value == value {
            return Some(t.clone())
        }

        if let Some(left) = &t.borrow().left {
            return Self::find_by_value(left.clone(), value)
        }
        if let Some(right) = &t.borrow().right {
            return Self::find_by_value(right.clone(), value)
        }

        None

    }
}

enum Action {
    ToOne,
    Split(u64, u64),
    Mul,
}

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

fn count_stones(stones: &Stones, blinks: u8) -> usize {
    let mut trees = vec![];
    for stone in stones.iter() {
        let mut node = Node::new(*stone);
        build_tree(node.clone(), node.clone(), 0, blinks);
        trees.push(node);
    }

    0
}

fn build_tree(root: TreeNode, node: TreeNode, depth: u8, max: u8) {
    if depth == max {
        return
    }

    let mut n = node.borrow_mut();
    let stone = n.value;
    let l = digit_length(stone);

    if stone == 0 {
        let on = n.add_left_child(1);
        build_tree(root.clone(), on, depth + 1, max);
    } else if l % 2 == 0 {
        let k = 10_u64.pow(l / 2);
        let l = stone / k;
        let r = stone - (l * k);

        let ln = n.add_left_child(l);
        let rn = n.add_right_child(r);
        build_tree(root.clone(), ln, depth + 1, max);
        build_tree(root.clone(), rn, depth + 1, max);
    } else {
        let mn = n.add_left_child(stone * 2024);
        build_tree(root.clone(), mn, depth + 1, max);
    }
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
