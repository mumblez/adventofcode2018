extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_input(file: &str) -> (HashMap<char, HashSet<char>>, Vec<char>) {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(
        r"Step (?P<p1>[A-Z]{1}) must be finished before step (?P<p2>[A-Z]{1}) can begin",
    )
    .unwrap();

    let mut steps: HashMap<char, HashSet<char>> = HashMap::new();
    let mut seen = HashSet::<char>::new();

    for line in reader.lines() {
        let input = line.unwrap();
        let capture = re.captures(&input).unwrap();
        let dependancy = capture["p1"].parse::<char>().unwrap();
        let step = capture["p2"].parse::<char>().unwrap();

        if !steps.contains_key(&step) {
            steps.insert(step, HashSet::<char>::new());
        }

        let step1 = steps.get_mut(&step).unwrap();
        step1.insert(dependancy);
        seen.insert(dependancy);
    }

    let mut starters = Vec::<char>::new();
    for c in &seen {
        if !steps.contains_key(&c) {
            starters.push(*c);
        }
    }
    starters.sort();
    (steps, starters)
}

fn main() {
    let (mut nodes, mut queue) = parse_input("input.txt");

    let mut answer = String::new();
    let mut node: char = queue[0];

    while !nodes.is_empty() {
        nodes.remove(&node);
        answer.push(node);
        if !queue.is_empty() {
            queue.remove(0);
        }
        for (_, d) in nodes.iter_mut() {
            if d.contains(&node) {
                d.remove(&node);
            }
        }
        for (p, d) in &nodes {
            if d.is_empty() {
                if !queue.contains(p) {
                    queue.push(*p);
                }
            }
        }
        queue.sort();
        if !queue.is_empty() {
            node = queue[0];
        }
    }

    println!("answer: {}", answer);
}
