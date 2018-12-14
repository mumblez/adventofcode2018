use failure::Error;
use piston_window::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Star {
    x: i64,
    y: i64,
    vx: i8,
    vy: i8,
}

impl Star {
    fn new(x: (i64, i64, i8, i8)) -> Star {
        Star {
            x: x.0,
            y: x.1,
            vx: x.2,
            vy: x.3,
        }
    }

    fn move_forwards(&mut self) {
        self.x += self.vx as i64;
        self.y += self.vy as i64;
    }

    fn move_backwards(&mut self) {
        self.x -= self.vx as i64;
        self.y -= self.vy as i64;
    }
}

fn parse_input(file: &str) -> Result<Vec<Star>, Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut stars = Vec::<Star>::new();

    let re = Regex::new(r"(-?\d{1,5}),\s+(-?\d{1,5}).*<\s*(-?\d{1,5}),\s+(-?\d{1,5})")?;

    for line in reader.lines() {
        let input = line.unwrap();
        let capture = re.captures(&input).unwrap();
        stars.push(Star::new((
            capture[1].parse::<i64>()?,
            capture[2].parse::<i64>()?,
            capture[3].parse::<i8>()?,
            capture[4].parse::<i8>()?,
        )));
    }
    Ok(stars)
}

fn main() -> Result<(), Error> {
    let mut stars = parse_input("input.txt")?;
    let (w_height, w_width) = (1000, 900);
    let mut window: PistonWindow = WindowSettings::new("Stars", [w_width, w_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let zoom = 4.0;
    let mut smallest_height = 100_u64;
    let approximate = (stars[0].x / stars[0].vx as i64).abs() - 50;
    let mut move_tally = 0u64;
    let mut shrink_complete = false;
    for star in stars.iter_mut() {
        for _ in 0..approximate {
            star.move_forwards();
        }
    }
    move_tally += approximate as u64;
    move_tally -= 1;

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([0.0; 4], graphics);
            for star in stars.iter() {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [
                        star.x as f64 * zoom,
                        star.y as f64 * zoom,
                        1.0 * zoom,
                        1.0 * zoom,
                    ],
                    context.transform,
                    graphics,
                );
            }
            for star in stars.iter_mut() {
                star.move_forwards();
            }
            if !shrink_complete {
                move_tally += 1;
            }
            let lowest_x = stars.iter().map(|x| x.x).min().unwrap();
            let highest_x = stars.iter().map(|x| x.x).max().unwrap();
            let tmp_height: u64 = (highest_x - lowest_x) as u64;
            if tmp_height < smallest_height {
                smallest_height = tmp_height;
            } else if smallest_height < 100 && tmp_height < 100 && tmp_height > smallest_height {
                // lock in place once it shrinks to lowest point and starts expanding!
                shrink_complete = true;
                for star in stars.iter_mut() {
                    star.move_backwards();
                }
            }
        });
    }
    println!("moves: {}", move_tally);
    Ok(())
}
