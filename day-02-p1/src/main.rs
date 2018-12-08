use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut twos: u64 = 0;
    let mut threes: u64 = 0;

    for line in reader.lines() {
        let mut seen: HashMap<char, u64> = HashMap::new();
        let sequence = line.unwrap();

        sequence.chars().for_each(|c| {
            let total = seen.entry(c).or_insert(0);
            *total += 1;
        });

        let mut two_exists: bool = false;
        let mut three_exists: bool = false;

        if seen.iter().filter(|(_, &v)| v == 2).count() > 0 {
            two_exists = true;
        }
        if seen.iter().filter(|(_, &v)| v == 3).count() > 0 {
            three_exists = true;
        }

        if two_exists {
            twos += 1;
        }
        if three_exists {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
    Ok(())
}
