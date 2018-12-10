use std::fs::File;
use std::io::prelude::*;

// not the fastest solution, can be optimised!
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    input = input.trim().to_string();

    loop {
        let mut tally = 0_usize;
        let input_clone = input.clone().trim().to_string();
        let mut modified = false;
        let mut skip = false;

        for (idx, c) in input_clone.chars().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            let polarity: char = if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            };

            if idx != (input_clone.len() - 1) {
                let next_char: char = input_clone.chars().nth(idx + 1).unwrap();
                if next_char == polarity {
                    input.remove(idx - tally);
                    input.remove(idx - tally);
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

    println!("{}", input.len());
    Ok(())
}
