extern crate rayon;

use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

fn polarity(c: &char) -> char {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

// not the fastest solution, can be optimised!
fn run_reaction(input: &String) -> u64 {
    let mut new_input = input.clone().trim().to_string();
    loop {
        let mut tally = 0_usize;
        let input_clone = new_input.clone().trim().to_string();
        let mut modified = false;
        let mut skip = false;

        for (idx, c) in input_clone.chars().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            if idx != (input_clone.len() - 1) {
                let next_char: char = input_clone.chars().nth(idx + 1).unwrap();
                if next_char == polarity(&c) {
                    new_input.remove(idx - tally);
                    new_input.remove(idx - tally);
                    tally += 2;
                    modified = true;
                    skip = true;
                }
            } else {
                break;
            }
        }
        if !modified {
            break;
        }
    }
    new_input.len() as u64
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    // let mut file = File::open("debug.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input = input.trim().to_string();
    // let mut lowest_total = 50000_u64;
    let lowest_total = Arc::new(Mutex::new(50000_u64));
    let units = "abcdefghijklmnopqrstuvwxyz";

    units.par_chars().for_each(|unit| {
        let mut input_copy = input.clone();
        input_copy.retain(|c| c != unit);
        input_copy.retain(|c| c != polarity(&unit));
        let result = run_reaction(&input_copy);
        let mut lt = lowest_total.lock().unwrap();
        if result < *lt {
            *lt = result;
            println!("total: {}", *lt);
        }
    });
    println!("answer {}", *lowest_total.lock().unwrap());
    Ok(())
}
