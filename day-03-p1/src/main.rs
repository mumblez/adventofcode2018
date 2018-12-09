use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn parse(line: &str) -> Claim {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    // extract x and y
    let mut xy_string: String = tokens[2].into();
    xy_string.remove(xy_string.find(':').unwrap());
    let x_y: Vec<&str> = xy_string.split(',').collect();

    // extract height and width
    let w_h: Vec<&str> = tokens[3].split('x').collect();

    return Claim {
        x: x_y[0].parse::<usize>().unwrap(),
        y: x_y[1].parse::<usize>().unwrap(),
        width: w_h[0].parse::<usize>().unwrap(),
        height: w_h[1].parse::<usize>().unwrap(),
    };
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut claims: Vec<Claim> = Vec::new();
    let mut grid = [[false; 1000]; 1000];
    let mut overlaps: HashSet<(usize, usize)> = HashSet::new();

    // read into memory
    for line in reader.lines() {
        let claim = parse(&line.unwrap());
        claims.push(claim);
    }

    for claim in claims {
        for x in (claim.x + 1)..((claim.x + 1) + claim.width) {
            for y in (claim.y + 1)..((claim.y + 1) + claim.height) {
                if grid[x][y] == false {
                    grid[x][y] = true;
                } else {
                    if !overlaps.contains(&(x, y)) {
                        overlaps.insert((x, y));
                    }
                }
            }
        }
    }

    println!("answer: {}", overlaps.len());

    Ok(())
}
