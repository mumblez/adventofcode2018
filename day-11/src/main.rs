use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn generate_grid(serial: i64) -> [[i8; 300]; 300] {
    let mut grid = [[0_i8; 300]; 300];

    for y in 0..300 {
        for x in 0..300 {
            let rack_id = x + 10 + 1;
            let mut power_level: u64 = rack_id * (y + 1);
            power_level = (power_level + serial as u64) * rack_id;
            let mut cell: i8 = ((power_level / 100) % 10) as i8;
            cell -= 5;
            grid[y as usize][x as usize] = cell as i8;
        }
    }
    grid
}

fn calculate_most_energy(grid: &[[i8; 300]; 300], size: usize) -> (usize, usize, u64) {
    let mut total = 0_u64;
    let (mut xx, mut yy) = (0_usize, 0_usize);
    let max = grid.len() - size;

    for y in 0..max {
        for x in 0..max {
            // sum cells
            let mut sub_total = 0_i64;
            for b in y..(y + size) {
                for a in x..(x + size) {
                    sub_total += grid[b][a] as i64;
                }
            }
            if sub_total > 0 && sub_total as u64 > total {
                total = sub_total as u64;
                xx = x;
                yy = y;
            }
        }
    }
    (xx + 1, yy + 1, total)
}

fn calculate_most_energy_and_size(grid: &[[i8; 300]; 300]) -> (usize, usize, u16) {
    let mut max_size = 0_u16;
    let mut max_x = 0_usize;
    let mut max_y = 0_usize;
    let mut max_total = 0_u64;
    let totals = Arc::new(Mutex::new(Vec::<(usize, usize, u64, u16)>::new()));

    (1..301).into_par_iter().for_each(|n| {
        let (x, y, total) = calculate_most_energy(&grid, n as usize);
        let tc = totals.clone();
        let mut t = tc.lock().unwrap();
        t.push((x, y, total, n as u16));
    });
    totals.lock().unwrap().iter().for_each(|x| {
        if x.2 > max_total {
            max_total = x.2;
            max_x = x.0;
            max_y = x.1;
            max_size = x.3;
        }
    });

    (max_x, max_y, max_size)
}

fn main() {
    let input = 8141;
    let grid = generate_grid(input);

    // p1
    // let (x, y, _) = calculate_most_energy(&grid, 3);
    // println!("part 1: x, y {},{}", x, y);
    // p2
    let (x, y, size) = calculate_most_energy_and_size(&grid);
    println!("part 2: x, y, size: {},{},{}", x, y, size);
}
