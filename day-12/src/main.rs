use failure::Error;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input(file: &str) -> Result<(HashSet<String>, String), Error> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut istate_found = false;
    let mut istate = String::new();
    // let mut patterns = Vec::<Pattern>::new();
    let mut patterns: HashSet<String> = HashSet::new();

    let re_state = Regex::new(r"initial state:\s+(?P<state>[#\.]+)")?;
    // let re_patterns = Regex::new(r"(?P<p1>[\.#]{5}) => (?P<p2>[\.#]{1})").unwrap();
    let re_patterns = Regex::new(r"(?P<p1>[\.#]{5}) => #").unwrap();

    for line in reader.lines() {
        let input = line.unwrap();
        if let Some(capture_patterns) = re_patterns.captures(&input) {
            // only add valid ones to a hashmap
            if let Ok(p) = capture_patterns["p1"].parse::<String>() {
                patterns.insert(p.to_owned());
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
    last_res = state.clone();
    let mut last_pattern = String::new();
    let mut pattern_repeat = 0;

    // make loop as tight as possible
    for g in 0..=150 as u64 {
        let mut gen = String::new();
        gen.push_str(last_res.as_str());
        last_idx += padding(&mut gen) as i64;
        let mut res = String::new();
        res.push_str("..");

        // can we parallellise this?
        (0..(gen.len() - 5)).for_each(|x| {
            if patterns.contains(&gen[x..x + 5]) {
                res.push('#');
            } else {
                res.push('.');
            }
        });

        last_res = res;
        // p1
        // if g == 20 {
        //     let mut total = 0_i64;
        //     gen.chars().enumerate().for_each(|(x, c)| {
        //         if c == '#' {
        //             // total += x as i64 - last_idx;
        //             total += x as i64 - last_idx;
        //         }
        //     });
        //     println!("total: {}", total)
        // }

        // p2
        let mut subtotal = 0_i64;
        gen.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                // total += x as i64 - last_idx;
                subtotal += x as i64 - last_idx;
            }
        });
        // seems to repeat after 118 onwards, with diff of 73
        // if we run up to 200 and save the last total (16376), then add to
        // (50,000,000,000 - 200) * 73 + 16376 = 3650000001776
        //
        // trim of preceeding '.' to make patterns obvious
        let first_plant = last_res.find('#').unwrap();
        print!("res: {}", &last_res[first_plant..]);
        println!(
            "g: {}, last_idx: {}, total: {}, diff: {}",
            g,
            last_idx,
            subtotal,
            subtotal - total
        );

        // record and compare against last pattern iteration
        if &last_res[first_plant..] == last_pattern {
            println!("pattern repeats!!!");
            pattern_repeat += 1;
            // ensure we repeat a few times consecutively
            if pattern_repeat > 5 {
                // now we can figure out p2 answer
                let answer: u64 =
                    (50_000_000_000 - g) * (subtotal - total) as u64 + subtotal as u64;
                println!("p2 answer: {}", answer);
                break;
            }
        } else {
            // reset if not consecutive!
            pattern_repeat = 0;
        }
        last_pattern.clear();
        last_pattern.push_str(&last_res[first_plant..]);

        total = subtotal;
    }

    Ok(())
}
