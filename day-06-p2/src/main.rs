// extern crate piston_window;
extern crate regex;

// use piston_window::*;
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u16,
    y: u16,
    area_count: u64,
    edge: bool,
}

fn parse_points(file: &str) -> Vec<Point> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(?P<p1>\d{1,3}), (?P<p2>\d{1,3})").unwrap();

    let mut points = Vec::<Point>::new();

    for line in reader.lines() {
        let input = line.unwrap();
        let capture = re.captures(&input).unwrap();
        points.push(Point {
            x: capture["p1"].parse::<u16>().unwrap(),
            y: capture["p2"].parse::<u16>().unwrap(),
            area_count: 0,
            edge: false,
        });
    }
    points
}

fn grid_size(points: &Vec<Point>) -> (u16, u16) {
    let mut largest_x = 0_u16;
    let mut largest_y = 0_u16;

    for point in points {
        if point.x > largest_x {
            largest_x = point.x
        };
        if point.y > largest_y {
            largest_y = point.y
        };
    }

    largest_x += 1;
    largest_y += 1;

    (largest_x, largest_y)
}

fn process_points(points: &mut Vec<Point>, x_len: u16, y_len: u16, region: u64) -> u64 {
    let mut region_tally = 0_u64;

    for y in 0..y_len {
        'point: for x in 0..x_len {
            let mut distances = Vec::<u64>::new();
            let mut point_region_tally = 0_u64;
            let mut point_coord = false;

            for point in points.iter_mut() {
                if point.x == x && point.y == y {
                    point.area_count += 1;
                    if x == 0 || y == 0 || x == x_len - 1 || y == y_len - 1 {
                        point.edge = true;
                    }
                    distances.push(0);
                    point_coord = true;
                }
                let distance =
                    (point.x as i64 - x as i64).abs() + (point.y as i64 - y as i64).abs();
                distances.push(distance as u64);
                point_region_tally += distance as u64;
            }

            if point_region_tally < region {
                region_tally += 1;
            }

            if point_coord {
                continue 'point;
            }

            let mut smallest_distance = 999999;
            let mut smallest_distance_idx = 0;
            for (idx, distance) in distances.iter().enumerate() {
                if *distance < smallest_distance {
                    smallest_distance = *distance;
                    smallest_distance_idx = idx;
                }
            }

            let mut duplicate_found = false;
            for (idx, distance) in distances.iter().enumerate() {
                if *distance == smallest_distance && smallest_distance_idx != idx {
                    duplicate_found = true
                }
            }

            if !duplicate_found {
                points[smallest_distance_idx].area_count += 1;
                if x == 0 || y == 0 || x == x_len - 1 || y == y_len - 1 {
                    points[smallest_distance_idx].edge = true;
                }
            }
        }
    }

    region_tally
}

fn main() -> std::io::Result<()> {
    let mut points = parse_points("input.txt");
    let (x_len, y_len) = grid_size(&points);
    let region_size = 10_000;

    let region_tally = process_points(&mut points, x_len, y_len, region_size);

    let mut largest_area = 0_u64;

    for point in &points {
        if !point.edge && point.area_count > largest_area {
            largest_area = point.area_count;
        }
    }

    println!("Largest area: {}", largest_area);
    println!("Region tally: {}", region_tally);

    Ok(())
}
