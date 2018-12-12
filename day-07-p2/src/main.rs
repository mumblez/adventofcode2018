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

fn alpha_index(c: &char) -> u64 {
    let a = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (a.find(*c).unwrap() + 61) as u64
}

enum Status {
    Working,
    Idle,
}

struct Worker {
    node: char,
    ticks: u64,
    status: Status,
}

impl Worker {
    fn new_workers(n: u64) -> Vec<Worker> {
        let mut workers = Vec::<Worker>::with_capacity(n as usize);
        for _ in 0..n {
            let worker = Worker {
                node: '.',
                ticks: 0,
                status: Status::Idle,
            };
            workers.push(worker);
        }
        workers
    }
    fn tick_down(&mut self) {
        if self.ticks != 0 {
            self.ticks -= 1;
        }
    }
}

fn main() {
    let (mut nodes, mut queue) = parse_input("input.txt");
    let worker_count = 5;
    let mut answer = String::new();
    let mut seconds = 0_u64;
    let mut workers = Worker::new_workers(worker_count);
    let mut in_progress = Vec::<char>::new();

    let mut ticks = true;
    while ticks {
        ticks = false;
        // push answer and update queue
        for w in workers.iter_mut() {
            if let Status::Working = w.status {
                if w.ticks == 0 {
                    w.status = Status::Idle;
                    answer.push(w.node);

                    // remove dependancies
                    for (_, d) in nodes.iter_mut() {
                        if d.contains(&w.node) {
                            d.remove(&w.node);
                        }
                    }

                    // push free nodes into queue
                    let mut to_remove = Vec::<char>::new();
                    for (p, d) in &nodes {
                        if d.is_empty() {
                            if !queue.contains(p) {
                                queue.push(*p);
                                to_remove.push(*p);
                            }
                        }
                    }

                    // remove from global nodes pushed values
                    for c in to_remove {
                        nodes.remove(&c);
                    }

                    // // remove from in_progress
                    if in_progress.contains(&w.node) {
                        let index = in_progress.iter().position(|&r| r == w.node).unwrap();
                        in_progress.remove(index);
                    }
                    queue.sort();
                    w.node = '.';
                }
            }
        }

        for _ in 0..queue.len() {
            // find idle worker and assign work
            let mut available = false;
            let mut w_idx = 0;
            for (idx, w) in workers.iter().enumerate() {
                if let Status::Idle = w.status {
                    available = true;
                    w_idx = idx;
                    break;
                }
            }
            if available {
                workers[w_idx].node = queue.remove(0);
                workers[w_idx].ticks = alpha_index(&workers[w_idx].node);
                workers[w_idx].status = Status::Working;
            }
        }

        // tick down
        for w in workers.iter_mut() {
            w.tick_down();
            if w.node != '.' {
                ticks = true;
            }
        }
        seconds += 1;
    }
    println!("answer: {}", seconds - 1);
}
