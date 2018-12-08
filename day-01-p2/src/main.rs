use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut seen = HashSet::new();
    let mut total: i64 = 0;

    seen.insert(0);

    loop {
        let file = File::open("input.txt")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let n: i64 = line.unwrap().parse().unwrap();
            total += n;
            if seen.contains(&total) {
                println!("{}", total);
                return Ok(());
            }
            seen.insert(total);
        }
    }
}
