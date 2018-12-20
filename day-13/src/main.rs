use failure::Error;
// use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
enum IntersectionDecision {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Cart {
    direction: Direction,
    intersection_decision: IntersectionDecision,
    x: usize,
    y: usize,
    crashed: bool,
    moved: bool,
}

impl Cart {
    // fn char(&self) -> char {
    //     match self.direction {
    //         Direction::Up => '^',
    //         Direction::Down => 'v',
    //         Direction::Left => '<',
    //         Direction::Right => '>',
    //     }
    // }

    fn advance(&mut self, grid: &Vec<Vec<char>>) {
        // which cell in front of
        let (new_y, new_x) = match self.direction {
            Direction::Up => (self.y - 1, self.x),
            Direction::Down => (self.y + 1, self.x),
            Direction::Left => (self.y, self.x - 1),
            Direction::Right => (self.y, self.x + 1),
        };

        // update direction
        match grid[new_y][new_x] {
            // advance and update direction
            // line -| - no need to change direction
            // corners \/
            '/' => {
                self.direction = match self.direction {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                }
            }
            '\\' => {
                self.direction = match self.direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                }
            }
            // intersection +
            '+' => {
                self.direction = match self.direction {
                    Direction::Left => match self.intersection_decision {
                        IntersectionDecision::Left => Direction::Down,
                        IntersectionDecision::Right => Direction::Up,
                        IntersectionDecision::Straight => Direction::Left,
                    },
                    Direction::Up => match self.intersection_decision {
                        IntersectionDecision::Left => Direction::Left,
                        IntersectionDecision::Right => Direction::Right,
                        IntersectionDecision::Straight => Direction::Up,
                    },
                    Direction::Right => match self.intersection_decision {
                        IntersectionDecision::Left => Direction::Up,
                        IntersectionDecision::Right => Direction::Down,
                        IntersectionDecision::Straight => Direction::Right,
                    },
                    Direction::Down => match self.intersection_decision {
                        IntersectionDecision::Left => Direction::Right,
                        IntersectionDecision::Right => Direction::Left,
                        IntersectionDecision::Straight => Direction::Down,
                    },
                };

                // update intersection_decision
                self.intersection_decision = match self.intersection_decision {
                    IntersectionDecision::Left => IntersectionDecision::Straight,
                    IntersectionDecision::Straight => IntersectionDecision::Right,
                    IntersectionDecision::Right => IntersectionDecision::Left,
                }
            }
            // do nothing
            _ => {}
        }

        // update x,y
        self.x = new_x;
        self.y = new_y;
    }
}

fn parse_input(file: &str) -> Result<(Vec<Vec<char>>, Vec<Cart>), Error> {
    // generate grid and also capture vehicles when found (v><^)
    // also don't ignore spaces
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut carts = Vec::<Cart>::new();

    // fill grid first before extracting carts
    for line in reader.lines() {
        let input = line.unwrap();
        let mut chars = Vec::<char>::new();
        input.chars().for_each(|x| chars.push(x));
        grid.push(chars);
    }

    // clean grid, extract cart and replace underlying char
    let cart_chars = "<v>^";
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if cart_chars.contains(grid[y][x]) {
                let cart_direction = match grid[y][x] {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => Direction::Up,
                };

                // cleanup grid
                grid[y][x] = match cart_direction {
                    Direction::Up => '|',
                    Direction::Down => '|',
                    Direction::Left => '-',
                    Direction::Right => '-',
                };

                carts.push(Cart {
                    direction: cart_direction,
                    intersection_decision: IntersectionDecision::Left,
                    x: x,
                    y: y,
                    crashed: false,
                    moved: false,
                });
            }
        }
    }

    Ok((grid, carts))
}

// fn update_grid(grid: &mut Vec<Vec<char>>, carts: &Vec<Cart>) {
//     let ch = "<>^v";
//     for c in carts {
//         if ch.contains(grid[c.y][c.x]) {
//             grid[c.y][c.x] = 'X';
//         } else {
//             grid[c.y][c.x] = c.char();
//         }
//     }
//     for x in 0..grid.len() {
//         println!("{}", &grid[x].iter().collect::<String>());
//     }
// }

fn main() -> Result<(), Error> {
    let (grid, mut carts) = parse_input("input.txt")?;

    let mut cartmap: HashSet<(usize, usize)> = carts.iter().map(|c| (c.x, c.y)).collect();
    // p1: 39,52
    // p2: last cart after removing all crashed!

    while carts.len() != 1 {
        carts.sort_by(|a, b| {
            if a.y == b.y && a.x < b.x {
                Ordering::Less
            } else if a.y == b.y && a.x == b.x {
                Ordering::Equal
            } else if a.y == b.y && a.x > b.x {
                Ordering::Greater
            } else {
                a.y.cmp(&b.y)
            }
        });

        // update_grid(&mut grid.clone(), &carts);
        let mut carts_to_remove = HashSet::<(usize, usize)>::new();

        // for c in carts.iter_mut() {
        while let Some(i) = carts.iter().position(|x| !x.crashed && !x.moved) {
            let mut cart = (carts[i].x, carts[i].y);
            cartmap.remove(&cart);
            carts[i].advance(&grid);
            carts[i].moved = true;
            cart = (carts[i].x, carts[i].y);

            if cartmap.contains(&cart) {
                // // p1
                // println!("crash at: {},{}, idx: {}", carts[i].x, carts[i].y, i);
                // break;
                // // p2
                // // record crashes and remove from our carts vec, continue iteration until last
                // // man standing

                // crash detected
                carts[i].crashed = true;
                cartmap.remove(&cart);
                carts_to_remove.insert(cart);
                // find and set the other cart crash status
                while let Some(n) = carts
                    .iter()
                    .position(|i| i.x == cart.0 && i.y == cart.1 && !i.crashed)
                {
                    carts[n].crashed = true;
                }
            } else {
                cartmap.insert(cart);
            }

            // ensure we process and advance our final cart in our current tick
            if carts.iter().filter(|x| !x.crashed).count() == 1 {
                carts
                    .iter_mut()
                    .filter(|x| !x.crashed)
                    .for_each(|x| x.advance(&grid));
                break;
            }
        }

        // after current tick processed, remove crashed carts and start again (optional)
        for (x, y) in carts_to_remove {
            while let Some(n) = carts.iter().position(|i| i.x == x && i.y == y) {
                carts.remove(n);
            }
        }

        // reset move status
        carts.iter_mut().for_each(|c| c.moved = false);
    }
    // update_grid(&mut grid.clone(), &carts);
    println!("last cart: {},{}", carts[0].x, carts[0].y);
    Ok(())
}
