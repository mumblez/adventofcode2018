use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    // let file = File::open("debug.txt")?;
    let reader = BufReader::new(file);

    let mut ids: Vec<String> = Vec::new();

    // read into memory
    for line in reader.lines() {
        let id = line.unwrap();
        ids.push(id);
    }

    // find pair where only 1 letter is different
    let mut counter: usize = 0;
    'id: for id in &ids {
        counter += 1;
        if counter > ids.len() {
            break;
        }

        let left: Vec<char> = id.chars().collect();
        let mut diffs: u64;
        let mut idx: usize;

        for n in counter..ids.len() {
            diffs = 0;
            idx = 0;
            let mut right: Vec<char> = ids[n].chars().collect();
            for (i, pair) in left.iter().zip(right.iter()).enumerate() {
                let (x, y) = pair;
                if x != y {
                    diffs += 1;
                    idx = i;
                }
                if diffs > 1 {
                    break;
                }
            }
            // we've found our match!
            if diffs == 1 {
                right.remove(idx);
                let answer: String = right.into_iter().collect();
                println!("{}", answer);
                break 'id;
            }
        }
    }
    Ok(())
}
