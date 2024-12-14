use std::fs;
use std::fs::File;
use std::{thread, time};
use std::io::{BufRead, BufReader, Write};
use image::{GenericImageView, ImageBuffer};

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
    move_robots(&mut robots, WIDTH, HEIGHT);
    println!("p1 {}", quadrant_product(&robots, WIDTH, HEIGHT));
    let mut robots = parse("input");
    move_robots_slowly(&mut robots, WIDTH, HEIGHT);
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

fn move_robots(robots: &mut Vec<Robot>, w: i16, h: i16) {
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
}

fn move_robots_slowly(robots: &mut Vec<Robot>, w: i16, h: i16) {
    for t in 0..10_000 {
        let mut imgbuf = ImageBuffer::new(w as u32, h as u32);

        for robot in robots.iter_mut() {
            robot.x += robot.vx;
            robot.y += robot.vy;

            let nx = robot.x.rem_euclid(w);
            let ny = robot.y.rem_euclid(h);
            robot.x = nx;
            robot.y = ny;
        }

        for y in 0..h {
            for x in 0..w {
                let c = robots
                    .iter()
                    .filter(&&|r: &&Robot| r.y == y && r.x == x)
                    .count();

                let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                if c > 0 {
                    let buf: [u16; 3] = [0, 0, 0];
                    *pixel = image::Rgb(buf);
                } else {
                    let buf: [u16; 3] = [255, 255, 255];
                    *pixel = image::Rgb(buf);
                }
            }
        }
        imgbuf.save(format!("{}.png", t)).unwrap();
    }
}

fn quadrant_product(robots: &Vec<Robot>, w: i16, h: i16) -> usize {
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
    move_robots_slowly(&mut robots, 11, 7);
    assert_eq!(quadrant_product(&robots, 11, 7), 12);

}
