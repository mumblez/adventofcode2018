use failure::Error;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Pattern {
    pattern: String,
    plant: bool,
}

fn parse_input(file: &str) -> Result<(Vec<Pattern>, String), Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut istate_found = false;
    let mut istate = String::new();
    let mut patterns = Vec::<Pattern>::new();

    let re_state = Regex::new(r"initial state:\s+(?P<state>[#\.]+)")?;
    let re_patterns = Regex::new(r"(?P<p1>[\.#]{5}) => (?P<p2>[\.#]{1})").unwrap();

    for line in reader.lines() {
        let input = line.unwrap();
        if let Some(capture_patterns) = re_patterns.captures(&input) {
            let mut plant_action = false;
            if let Ok(p) = capture_patterns["p2"].parse::<String>() {
                if p == "#" {
                    plant_action = true;
                }
            }

            if let Ok(p) = capture_patterns["p1"].parse::<String>() {
                let pot = Pattern {
                    pattern: p,
                    plant: plant_action,
                };
                patterns.push(pot);
            }
        }

        if !istate_found {
            let capture_state = re_state.captures(&input).unwrap();
            if let Ok(state) = capture_state["state"].parse::<String>() {
                istate = state;
                istate_found = true;
            }
        }
    }

    Ok((patterns, istate))
}

fn padding(gen: &mut String) -> u64 {
    // add appropriate padding to both ends
    // if padding on left, return by how much so we can increment index position in string
    // ensure 4 empty pots on first and last found planted pots
    let mut idx = 0_u64;
    let max_pad = 4;
    let empty_pot = '.';
    let first_plant = gen.find('#').unwrap();
    let last_plant = gen.rfind('#').unwrap();

    // right padding
    (0..(4 - (gen.len() - last_plant - 1))).for_each(|_| gen.push(empty_pot));
    // left padding
    if first_plant <= 4 {
        (0..(max_pad - first_plant as u64)).for_each(|_| {
            gen.insert(0, empty_pot);
            idx += 1;
        });
    }
    idx
}

fn main() -> Result<(), Error> {
    let (patterns, state) = parse_input("input.txt")?;
    // let (patterns, state) = parse_input("debug.txt")?;

    let mut last_idx = 0_i64;
    let mut total = 0_i64;
    let mut last_res = String::new();

    // for g in 0..=20 as u64 {
    // p1
    for g in 0..=50_000_000_000 as u64 {
        // p2
        let mut gen = String::new();
        if g == 0 {
            gen = state.clone();
        } else {
            gen.push_str(last_res.as_str());
        }
        let idx_shift = padding(&mut gen);
        last_idx += idx_shift as i64;
        let mut res = String::new();
        res.push_str("..");

        // maybe convert patterns to hashmap to improve performance?
        (0..(gen.len() - 5)).for_each(|x| {
            let mut pattern_found = false;
            patterns.iter().for_each(|p| {
                if p.pattern == &gen[x..x + 5] {
                    pattern_found = true;
                    if p.plant {
                        res.push('#');
                    } else {
                        res.push('.');
                    }
                }
            });
            if !pattern_found {
                res.push('.');
            }
        });

        // calculate on last run only
        // if g == 20 {
        // p1
        if g == 50_000_000_000 {
            // p2
            gen.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    total += x as i64 - last_idx;
                }
            });
            println!("");
            println!("total: {}", total);
        }
        last_res = res;
    }

    Ok(())
}
