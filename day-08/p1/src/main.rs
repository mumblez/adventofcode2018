use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input(input: &str) -> Result<Vec<u64>, Error> {
    let file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect())
}

fn main() -> Result<(), Error> {
    // let input = parse_input("debug.txt");
    let input = parse_input("input.txt")?;
    let mut answer: u64 = 0;

    // instead of building a tree / graph we'll track indexes as we're reading the data serially

    // trackers
    let mut childs = Vec::<u64>::new();
    let mut data = Vec::<u64>::new();
    let mut i = 0_usize;

    loop {
        let x = input[i];

        // if 0, read values and advance i past data
        if x == 0 {
            i += 1;
            for _ in 0..input[i] {
                i += 1;
                answer += input[i];
            }
            while let Some(last) = childs.last_mut() {
                *last -= 1;
                if *last == 0 {
                    childs.pop().unwrap();
                    // read remaining data
                    if let Some(last_data) = data.last() {
                        for _ in 0..*last_data {
                            i += 1;
                            answer += input[i];
                        }
                        data.pop();
                    }
                } else {
                    break;
                }
            }
        } else {
            childs.push(input[i]);
            i += 1;
            data.push(input[i]);
        }
        i += 1;

        if i == input.len() {
            break;
        }
    }

    // answer
    println!("answer: {}", answer);
    Ok(())
}
