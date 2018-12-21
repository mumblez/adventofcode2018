fn p1(grid: &mut Vec<u8>, elves: &mut [u64], after_count: &usize) {
    while grid.len() < (*after_count + 10) {
        let new_recipe = grid[elves[0] as usize] + grid[elves[1] as usize];

        if new_recipe < 10 {
            grid.push(new_recipe);
        } else {
            grid.push(new_recipe / 10);
            grid.push(new_recipe % 10);
        }

        for elf in elves.iter_mut() {
            let movement = (grid[*elf as usize] + 1) as u64;
            let gl = grid.len() as u64;
            *elf = if movement > gl - (*elf as u64 + 1) {
                (movement - (gl - *elf as u64)) % gl
            } else {
                movement + *elf
            };
        }
    }

    // p1
    grid.iter()
        .skip(*after_count)
        .take(10)
        .for_each(|x| print!("{}", x));
    println!("");
}

fn check(grid: &Vec<u8>, ac: &Vec<u8>) -> bool {
    let gl = grid.len();
    if gl > 6 {
        let s = grid.get((gl - 6)..gl).unwrap();
        if *ac == s {
            println!("index: {}", gl - 6);
            true
        } else {
            false
        }
    } else {
        false
    }
}
fn main() {
    let mut grid = Vec::<u8>::with_capacity(10_000_000_000_000);
    grid.push(3);
    grid.push(7);
    let mut elves: [u64; 2] = [0, 1];
    let ac: Vec<u8> = vec![7, 6, 5, 0, 7, 1];
    // p1(&mut grid, &mut elves, &after_count);

    // p2
    loop {
        let new_recipe = grid[elves[0] as usize] + grid[elves[1] as usize];

        if new_recipe < 10 {
            grid.push(new_recipe);
            if check(&grid, &ac) {
                break;
            };
        } else {
            grid.push(new_recipe / 10);
            if check(&grid, &ac) {
                break;
            };
            grid.push(new_recipe % 10);
            if check(&grid, &ac) {
                break;
            };
        }

        for elf in elves.iter_mut() {
            let movement = (grid[*elf as usize] + 1) as u64;
            let gl = grid.len() as u64;
            *elf = if movement > gl - (*elf as u64 + 1) {
                (movement - (gl - *elf as u64)) % gl
            } else {
                movement + *elf
            };
        }
    }
}
