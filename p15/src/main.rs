use std::fs;
use std::collections::VecDeque;

type RawGrid = Vec<Vec<char>>;

struct Grid {
    vector: RawGrid,
    ylen: isize,
    xlen: isize
}

impl Grid {
    fn new(vector: RawGrid) -> Grid {
        let ylen = vector.len() as isize;
        let xlen = vector[0].len() as isize;

        Grid {vector, ylen, xlen}
    }

    fn translation(d: char) -> (isize, isize) {
        match d {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _   => (0, 0)
        }
    }

    fn move_node(&mut self, d: char, y: isize, x: isize) -> (isize, isize) {
        let (ty, tx) = Self::translation(d);

        let ny = y + ty;
        let nx = x + tx;
        let c = self.get(ny, nx);

        if c == '.' {
            self.swap(y, x, ny, nx);
            return (ny, nx)
        }

        if c == 'O' {
            let mut oy = ny;
            let mut ox = nx;

            loop {
                oy += ty;
                ox += tx;

                let c = self.get(oy, ox);

                if c == '#' {
                    return (y, x);
                } else if c == '.' {
                    self.swap(ny, nx, oy, ox);
                    break;
                }
            }

            self.swap(y, x, ny, nx);
            return (ny, nx);
        }

        return (y, x)
    }

    fn move_nodes(&mut self, d: char, y: isize, x: isize) -> (isize, isize) {
        let (ty, tx) = Self::translation(d);
        let ny = y + ty;
        let nx = x + tx;
        let c = self.get(ny, nx);

        if c == '#' || d == '\n' {
            return (y, x);
        }

        if c == '.' {
            self.swap(y, x, ny, nx);
            return (ny, nx)
        }

        let mut coords = vec![];
        let mut positions = VecDeque::new();
        positions.push_front((ny, nx));

        while let Some((dy, dx)) = positions.pop_front() {
            if coords.contains(&(dy, dx)) {
                continue
            }

            let (tty, ttx) = (dy + ty, dx + tx);
            let c = self.get(dy, dx);

            if c != ']' && c != '[' {
                continue
            }

            let ex = if c == ']' {
                -1
            } else if c == '[' {
                1
            } else {
                0
            };

            coords.push((dy, dx));
            positions.push_back((tty, ttx));
            positions.push_front((dy, dx + ex));
        }

        // Test if any of the moves ends up in a wall
        for (my, mx) in coords.iter() {
            if self.get(my + ty, mx + tx) == '#' {
                return (y, x);
            }
        }

        for (my, mx) in coords.iter().rev() {
            self.swap(*my, *mx, my + ty, mx + tx);
        }

        self.swap(y, x, y + ty, x + tx);

        (ny, nx)
    }

    fn get(&self, y: isize, x: isize) -> char {
        self.vector[y as usize][x as usize]
    }

    fn swap(&mut self, ay: isize, ax: isize, by: isize, bx: isize) {
        let ac = self.get(ay, ax);
        let bc = self.get(by, bx);

        self.vector[ay as usize][ax as usize] = bc;
        self.vector[by as usize][bx as usize] = ac;
    }

    fn robot(&self) -> (isize, isize) {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                if self.get(y, x) == '@' {
                    return (y, x)
                }
            }
        }

        panic!("No robot found")
    }

    fn expand(&self) -> Grid {
        let mut input = vec![];
        for y in 0..self.ylen {
            let mut row = vec![];
            for x in 0..self.xlen {
                let mut appendix = match self.get(y, x) {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => panic!("Invalid character")
                };

                row.append(&mut appendix);
            }
            input.push(row);
        }

        Grid::new(input)
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                print!("{}", self.vector[y as usize][x as usize]);
            }
            println!("");
        }
    }
}

fn main() {
    let (mut grid, directions) = parse("input");
    let mut expanded_grid = grid.expand();

    println!("p1 {}", move_boxes(&mut grid, &directions, 'O'));
    println!("p2 {}", move_boxes(&mut expanded_grid, &directions, '['));
}

fn parse(input: &'static str) -> (Grid, String) {
    let raw = fs::read_to_string(input).unwrap();
    let (grid, directions) = raw.split_once("\n\n").unwrap();
    let vector = grid.lines().map(|line| {
        line.chars().collect()
    }).collect();

    (Grid::new(vector), String::from(directions.trim()))
}

fn move_boxes(grid: &mut Grid, dir: &String, search: char) -> isize {
    let (mut starty, mut startx) = grid.robot();

    for d in dir.chars() {
        (starty, startx) = if search == 'O' {
            grid.move_node(d, starty, startx)
        } else {
            grid.move_nodes(d, starty, startx)
        };
    }

    (0..grid.ylen).map(|y| {
        (0..grid.xlen)
            .filter(|&x| grid.get(y, x) == search)
            .map(|x| 100 * y + x)
            .sum::<isize>()
    }).sum()
}

#[test]
fn test_move_boxes() {
    let (mut grid, directions) = parse("2");
    assert_eq!(move_boxes(&mut grid, &directions, 'O'), 2028);

    let (mut grid, directions) = parse("1");
    assert_eq!(move_boxes(&mut grid, &directions, 'O'), 10092);

    let (grid, directions) = parse("3");
    let mut expanded_grid = grid.expand();
    assert_eq!(move_boxes(&mut expanded_grid, &directions, '['), 618);

    let (grid, directions) = parse("1");
    let mut expanded_grid = grid.expand();
    assert_eq!(move_boxes(&mut expanded_grid, &directions, '['), 9021);
}
