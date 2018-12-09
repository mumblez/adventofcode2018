use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn parse(line: &str) -> Claim {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    // extract claim id
    let mut id_string: String = tokens[0].into();
    id_string.remove(id_string.find('#').unwrap());

    // extract x and y
    let mut xy_string: String = tokens[2].into();
    xy_string.remove(xy_string.find(':').unwrap());
    let x_y: Vec<&str> = xy_string.split(',').collect();

    // extract height and width
    let w_h: Vec<&str> = tokens[3].split('x').collect();

    return Claim {
        id: id_string.parse::<usize>().unwrap(),
        x: x_y[0].parse::<usize>().unwrap(),
        y: x_y[1].parse::<usize>().unwrap(),
        width: w_h[0].parse::<usize>().unwrap(),
        height: w_h[1].parse::<usize>().unwrap(),
    };
}

fn count_id(claim: &Claim, grid: &Vec<[usize; 1000]>, overlap_grid: &Vec<[bool; 1000]>) -> usize {
    let mut count = 0_usize;
    for x in (claim.x + 1)..((claim.x + 1) + claim.width) {
        for y in (claim.y + 1)..((claim.y + 1) + claim.height) {
            if grid[x][y] == claim.id && overlap_grid[x][y] == false {
                count += 1;
            } else {
                return 0;
            }
        }
    }
    count
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut claims: Vec<Claim> = Vec::new();
    let mut grid = vec![[0_usize; 1000]; 1000];
    let mut overlap_grid = vec![[false; 1000]; 1000];

    // read into memory
    for line in reader.lines() {
        let claim = parse(&line.unwrap());
        claims.push(claim);
    }

    // populate grids
    for claim in &claims {
        for x in (claim.x + 1)..((claim.x + 1) + claim.width) {
            for y in (claim.y + 1)..((claim.y + 1) + claim.height) {
                if grid[x][y] != 0 {
                    overlap_grid[x][y] = true;
                }
                grid[x][y] = claim.id;
            }
        }
    }

    // scan grid and count if total = claim width x height
    for claim in &claims {
        if (claim.width * claim.height) == count_id(&claim, &grid, &overlap_grid) {
            println!("Claim with no overlapping: {}", claim.id);
            break;
        }
    }

    Ok(())
}
