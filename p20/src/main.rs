use std::cmp::Ordering;
use std::ops::Range;
use std::fs;
use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [Point; 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];

type Point = (i16, i16);

struct Grid {
    vector: Vec<Vec<char>>,
    size: usize
}

impl Grid {
    fn new(vector: Vec<Vec<char>>) -> Grid {
        let size = vector.len();
        Grid { size, vector }
    }

    fn is_wall(&self, p: &Point) -> bool {
        self.vector[p.0 as usize][p.1 as usize] == '#'
    }

    fn out_of_bounds(&self, p: &Point) -> bool {
        let s = self.size as i16;

        p.0 < 0 || p.1 < 0 || p.0 >= s || p.1 >= s
    }

    fn id(&self, p: &Point) -> usize {
        let y = p.0 as usize;
        let x = p.1 as usize;

        (y * self.size) + x
    }

    fn to_point(&self, id: usize) -> Point {
        let y = id / self.size;
        let x = id % self.size;

        (y as i16, x as i16)
    }

    fn id_is(&self, id: usize, search: char) -> bool {
        let y = id / self.size;
        let x = id % self.size;

        if self.out_of_bounds(&(y as i16, x as i16)) {
            return false
        }

        self.vector[y][x] == search
    }

    fn lookup(&self, search: char) -> Point {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.vector[y][x] == search {
                    return (y as i16, x as i16)
                }
            }
        }
        return (0, 0)
    }

    #[allow(dead_code)]
    fn debug(&self, points: &Vec<(usize, bool)>) {
        for y in 0..self.size {
            for x in 0..self.size {
                let did = self.id(&(y as i16, x as i16));

                if let Some((_, cheated)) = points.iter().find(|(id, _)| did == *id) {
                    if *cheated {
                        print!("C");
                    } else {
                        print!("X");
                    }
                } else {
                    print!("{}", self.vector[y][x]);
                }
            }
            println!("");
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: Point,
    cost: usize, // g-score: cost from start to this node
    estimate: usize, // f-score: g-score + heuristic estimate to goal
    cheat_ps: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate) // Reverse ordering for min-heap behavior
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let grid = parse("input");

    println!("p1 {}", cheat_count(&grid, 100));
    println!("p2 {}", cheat_count_revised(&grid, 100));
}

fn parse(input: &'static str) -> Grid {
    let vector = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Grid::new(vector)
}

// This is the Manhattan distance
fn heuristic(s: &Point, e: &Point) -> usize {
    let dx = if s.0 < e.0 { e.0 - s.0 } else { s.0 - e.0 };
    let dy = if s.1 < e.1 { e.1 - s.1 } else { s.1 - e.1 };
    (dx + dy) as usize
}

fn reconstruct_path(map: &HashMap<usize, usize>, id: usize) -> Vec<isize> {
    let mut route = vec![];
    let mut search = id;
    loop {
        let prev_id = map.get(&search);
        match prev_id {
            Some(x) => {
                route.push(*x as isize);
                search = *x;
            },
            None => break
        }
    }
    route
}

// Basic A* implementation
fn route(grid: &Grid, cheat: &Vec<usize>) -> Option<Vec<isize>> {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut g_score = vec![usize::MAX; grid.size * grid.size];

    heap.push(Node {
        position: start,
        cost: 0,
        estimate: heuristic(&start, &end),
        cheat_ps: 0
    });

    g_score[grid.id(&start)] = 0;

    while let Some(node) = heap.pop() {
        let id = grid.id(&node.position);
        if node.position == end {
            return Some(reconstruct_path(&came_from, id));
        }

        for (ty, tx) in &DIRECTIONS {
            let np = (node.position.0 + ty, node.position.1 + tx);

            if grid.out_of_bounds(&np) {
                continue
            }

            let new_score = g_score[id] + 1;
            let next_id = grid.id(&np);

            if grid.is_wall(&np) && !cheat.contains(&next_id) {
                continue
            }

            if new_score < g_score[next_id] {
                came_from.insert(next_id, id);
                g_score[next_id] = new_score;
                heap.push(Node {
                    position: np,
                    cost: new_score,
                    estimate: new_score + heuristic(&np, &end),
                    cheat_ps: 0
                });
            }
        }
    }

    None
}

fn cheat_count(grid: &Grid, seconds: usize) -> usize {
    let regular = route(grid, &vec![]).unwrap();
    let t_no_cheating = regular.len();

    let mut count = 0;
    let mut cheats = HashSet::new();
    let mut starts = HashSet::new();

    let grid_size = grid.size as isize;
    let tries = vec![-1, 1, -grid_size, grid_size];

    for r in regular {
        for t in &tries {
            let q = (r + *t) as usize;

            if grid.id_is(q, '#') {
                starts.insert(q);
            }
        }
    }

    for s in starts {
        let t = s as isize;

        for q in &tries {
            let r = (t + q) as usize;
            if grid.id_is(r, '.') {
                cheats.insert(vec![s, r]);
                break;
            }
        }
    }

    for cheat in cheats.iter() {
        let cheated_route = route(grid, cheat).unwrap();
        let saved = t_no_cheating - cheated_route.len();

        if saved >= seconds {
            count += 1;
        }
    }

    count
}

#[test]
fn test_cheat_count() {
    let grid = parse("1");
    assert_eq!(cheat_count(&grid, 12), 8);
}

fn reconstruct_path_with_hacks(
    map: &HashMap<usize, usize>,
    hacks: &HashSet<usize>,
    id: usize) -> Vec<(usize, bool)> {

    let mut route = vec![];
    let mut search = id;
    loop {
        let prev_id = map.get(&search);
        match prev_id {
            Some(x) => {
                route.push((*x, hacks.contains(x)));
                search = *x;
            },
            None => break
        }
    }
    route
}

// Basic A* implementation
fn route_range(grid: &Grid, skip: usize) -> Option<Vec<(usize, bool)>> {
    let start = grid.lookup('S');
    let end = grid.lookup('E');
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<usize, usize> = HashMap::new();
    let mut hacked = HashSet::new();
    let mut g_score = vec![usize::MAX; grid.size * grid.size];

    heap.push(Node {
        position: start,
        cost: 0,
        estimate: heuristic(&start, &end),
        cheat_ps: 0
    });

    g_score[grid.id(&start)] = 0;

    while let Some(node) = heap.pop() {
        let id = grid.id(&node.position);

        if node.position == end {
            return Some(
                reconstruct_path_with_hacks(
                    &came_from,
                    &hacked,
                    id
                )
            );
        }

        for (ty, tx) in &DIRECTIONS {
            let np = (node.position.0 + ty, node.position.1 + tx);

            if grid.out_of_bounds(&np) {
                continue
            }

            let new_score = g_score[id] + 1;
            let next_id = grid.id(&np);
            let mut ps = node.cheat_ps;

            if grid.is_wall(&np)  {
                if next_id == skip || ps > 0 {
                    ps += 1;
                }

                if ps == 0 || ps == 20 {
                    continue
                }
            }

            if new_score < g_score[next_id] {
                came_from.insert(next_id, id);
                g_score[next_id] = new_score;

                heap.push(Node {
                    position: np,
                    cost: new_score,
                    estimate: new_score + heuristic(&np, &end),
                    cheat_ps: ps
                });
            }
        }
    }

    None
}

#[test]
fn test_falls_within() {
    assert_eq!(falls_within(&12, &(6, 13), 5), true);
    assert_eq!(falls_within(&2, &(6, 13), 5), false);
    assert_eq!(falls_within(&13, &(6, 13), 5), true);
}

use std::{thread, time};

fn cheat_count_revised(grid: &Grid, seconds: usize) -> usize {
    let start = grid.lookup('S');
    let goal = grid.lookup('E');
    let regular = route(grid, &vec![]).unwrap();
    let t_no_cheating = regular.len();

    let mut count = 0;
    let mut starts = HashSet::new();

    let grid_size = grid.size as isize;
    let tries = vec![-1, 1, -grid_size, grid_size];

    for r in regular {
        for t in &tries {
            let q = (r + *t) as usize;

            if grid.id_is(q, '#') {
                starts.insert(q);
            }
        }
    }

    for s in &starts {
        let n = route_range(grid, *s).unwrap();

        if n.len() < t_no_cheating {
            let diff = t_no_cheating - n.len();
            if diff > seconds {
                println!("{:?}", diff);
            }
        }
    }
    //println!("{:?}", cheats.len());
    //let mut map = HashMap::new();
    //let mut set = HashSet::new();
    //for cheat in cheats.iter() {
    //    let cheated_route = route_range(grid, cheat).unwrap();
    //    let saved = t_no_cheating - cheated_route.len();

    //    if saved >= seconds {
    //        let mut cheat_modded = *cheat;
    //        let n = cheated_route.iter().find(|(_, w)| *w);

    //        if let Some((id, _)) = n {
    //            cheat_modded.1 = *id;
    //        }
    //        println!("{:?} {:?}", cheat, cheat_modded);
    //        set.insert((cheat_modded, saved));
    //        grid.debug(&cheated_route);
    //        map.entry(saved).and_modify(|c| *c += 1).or_insert(1);
    //        count += 1;
    //    }
    //}

    //println!("Total unique cheats: {:?}", set.len());

    //for (k, v) in &map {
    //    println!("There are {:?} cheats that save {} picoseconds.", v, k);
    //}


    count
}

#[test]
fn test_cheat_count_no_revised() {
    let grid = parse("1");
    assert_eq!(cheat_count_revised(&grid, 50), 285);
}
