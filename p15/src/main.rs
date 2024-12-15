use std::fs;

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

    fn move_node(&mut self, d: char, y: isize, x: isize) -> (isize, isize) {
        let (ty, tx) = match d {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => return (y, x),
        };

        let ny = y + ty;
        let nx = x + tx;

        match self.get(ny, nx) {
            '.' => {
                self.swap(y, x, ny, nx);
                (ny, nx)
            },
            'O' => {
                let mut oy = ny;
                let mut ox = nx;

                loop {
                    oy += ty;
                    ox += tx;

                    match self.get(oy, ox) {
                        '.' => {
                            self.swap(ny, nx, oy, ox);
                            break;
                        },
                        'O' => continue,
                        '#' => return (y, x),
                        _ => panic!("Invalid character")
                    }
                }
                self.swap(y, x, ny, nx);
                (ny, nx)
            },
            '#' => (y, x),
            _ => {
                panic!("Invalid character")
            }
        }
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
            for x in 0..self.ylen {
                if self.get(y, x) == '@' {
                    return (y, x)
                }
            }
        }

        panic!("No guard found")
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for y in 0..self.ylen {
            for x in 0..self.ylen {
                print!("{}", self.vector[y as usize][x as usize]);
            }
            println!("");
        }
    }
}

fn main() {
    let (mut grid, directions) = parse("input");
    println!("p1 {}", move_boxes(&mut grid, &directions));
    //println!("p2 {}", valid_obstacle_count(&grid));
}

fn parse(input: &'static str) -> (Grid, String) {
    let raw = fs::read_to_string(input).unwrap();
    let (grid, directions) = raw.split_once("\n\n").unwrap();

    let vector = grid.lines().map(|line| {
        line.chars().collect()
    }).collect();

    (Grid::new(vector), String::from(directions.trim()))
}

fn move_boxes(grid: &mut Grid, directions: &String) -> isize {
    let (mut starty, mut startx) = grid.robot();

    for d in directions.chars() {
        let (ty, tx) = grid.move_node(d, starty, startx);
        starty = ty;
        startx = tx;
    }

    let mut subtotal = 0;
    for y in 0..grid.ylen {
        for x in 0..grid.xlen {
            if grid.get(y, x) != 'O' {
               continue
            }

            subtotal += 100 * y + x;
        }

    }

    subtotal
}

#[test]
fn test_move_boxes() {
    let (mut grid, directions) = parse("2");
    assert_eq!(move_boxes(&mut grid, &directions), 2028);
    let (mut grid, directions) = parse("1");
    assert_eq!(move_boxes(&mut grid, &directions), 10092)
}
