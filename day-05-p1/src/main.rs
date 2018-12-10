use std::fs::File;
use std::io::prelude::*;

fn run_reaction(input: &[u8]) -> u64 {
    let mut input_bytes = input.to_owned();
    let input_bytes_len = input_bytes.len();
    loop {
        let mut modified = false;

        // for (left_idx, c) in input_bytes.iter_mut().enumerate() {
        for left_idx in 0..input_bytes.len() {
            // skip left_idx if on space
            if input_bytes[left_idx] == b' ' {
                continue;
            }

            // ensure right_idx is always after left_idx
            let mut right_idx = left_idx + 1;

            // move right_idx along, skipping spaces
            while right_idx != input_bytes_len {
                if input_bytes[right_idx] == b' ' {
                    right_idx += 1;
                } else {
                    break;
                }
            }

            if right_idx != input_bytes_len {
                let cl = input_bytes[left_idx] as char;
                let cr = input_bytes[right_idx] as char;
                let polar = if cl.is_ascii_uppercase() {
                    cl.to_ascii_lowercase()
                } else {
                    cl.to_ascii_uppercase()
                };

                if cr == polar {
                    input_bytes[left_idx] = b' ';
                    input_bytes[right_idx] = b' ';
                    modified = true;
                }
            } else {
                break;
            }
        }
        if !modified {
            break;
        }
    }
    input_bytes.iter().filter(|x| **x != b' ').count() as u64
}

// not the fastest solution, can be optimised!
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    // let mut file = File::open("debug.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let input_bytes = input.trim().to_string().into_bytes();

    let res = run_reaction(&input_bytes);
    println!("{}", res);
    Ok(())
}
