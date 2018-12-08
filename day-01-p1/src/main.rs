use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total: i64 = 0;
    for line in reader.lines() {
        let n: i64 = line.unwrap().parse().unwrap();
        total += n;
    }
    println!("{}", total);
    Ok(())
}
