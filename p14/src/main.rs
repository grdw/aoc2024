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
    let robots = parse("input");
    println!("p1 {}", robot_positions(&robots, WIDTH, HEIGHT));
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

fn robot_positions(robots: &Vec<Robot>, w: i16, h: i16) -> usize {
    println!("{:?}", robots);
    0
}

#[test]
fn test_robot_positions() {
    let robots = parse("1");
    assert_eq!(robot_positions(&robots, 11, 7), 12);

}
