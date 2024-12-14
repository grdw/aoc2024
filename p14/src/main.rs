use std::fs;

const WIDTH: i16 = 101;
const HEIGHT: i16 = 103;
const TIME: usize = 100;

#[derive(Debug)]
struct Robot {
    x: i16,
    y: i16,
    vx: i16,
    vy: i16
}

fn main() {
    let mut robots = parse("input");
    println!("p1 {}", robot_positions(&mut robots, WIDTH, HEIGHT));
}

fn parse(input: &'static str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in fs::read_to_string(input).unwrap().lines() {
        let (pr, vr) = line.split_once(" ").unwrap();
        let (x, y) = parse_point(pr);
        let (vx, vy) = parse_point(vr);
        robots.push(Robot { x, y, vx, vy });
    }
    robots
}

fn parse_point(input: &str) -> (i16, i16) {
    let (_, r) = input.split_once("=").unwrap();
    let (xr, yr) = r.split_once(",").unwrap();
    let x = xr.parse::<i16>().unwrap();
    let y = yr.parse::<i16>().unwrap();

    (x, y)
}

fn robot_positions(robots: &mut Vec<Robot>, w: i16, h: i16) -> usize {
    for _ in 0..TIME {
        for robot in robots.iter_mut() {
            robot.x += robot.vx;
            robot.y += robot.vy;
        }
    }

    for robot in robots.iter_mut() {
        let nx = robot.x.rem_euclid(w);
        let ny = robot.y.rem_euclid(h);
        robot.x = nx;
        robot.y = ny;
    }

    let hh = h / 2;
    let wh = w / 2;
    let mut quadrants = vec![0; 4];

    for y in 0..h {
        for x in 0..w {
            let c = robots
                .iter()
                .filter(&&|r: &&Robot| r.y == y && r.x == x)
                .count();

            if c == 0 {
                continue
            }

            let q = if y < hh && x < wh {
                0
            } else if y < hh && x > wh {
                1
            } else if y > hh && x < wh {
                2
            } else if y > hh && x > wh {
                3
            } else {
                continue
            };

            quadrants[q] += c;
        }
    }

    quadrants.iter().product()
}

#[test]
fn test_robot_positions() {
    let mut robots = parse("1");
    assert_eq!(robot_positions(&mut robots, 11, 7), 12);

}
