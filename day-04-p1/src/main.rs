// we sorted input into chronological order before running the program!
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Guard {
    sleep_total: u64,
    minute_asleep: HashMap<u64, u64>,
}

fn main() -> std::io::Result<()> {
    // cheat and sort input `sort input.txt > sorted_input.txt`
    let file = File::open("sorted_input.txt")?;
    let reader = BufReader::new(file);
    let mut guards = HashMap::<u64, Guard>::new();

    let re_guard_id = Regex::new(r"\#(?P<id>\d{1,4})").unwrap();
    let re_minute = Regex::new(r":(?P<minute>\d{2})").unwrap();

    let mut current_guard: u64 = 0;
    let mut sleep_start: u64 = 0;
    let mut sleep_end: u64 = 0;
    for line in reader.lines() {
        let input = line.unwrap();

        if input.contains("Guard") {
            current_guard = re_guard_id.captures(&input).unwrap()["id"]
                .parse::<u64>()
                .unwrap();
            if !guards.contains_key(&current_guard) {
                guards.insert(
                    current_guard,
                    Guard {
                        sleep_total: 0,
                        minute_asleep: HashMap::new(),
                    },
                );
            }
        } else if input.contains("asleep") {
            sleep_start = re_minute.captures(&input).unwrap()["minute"]
                .parse::<u64>()
                .unwrap();
        } else if input.contains("wakes") {
            sleep_end = re_minute.captures(&input).unwrap()["minute"]
                .parse::<u64>()
                .unwrap();
            let mut guard = guards.get_mut(&current_guard).unwrap();
            guard.sleep_total += (sleep_end - sleep_start);
            for minute in sleep_start..sleep_end {
                let min = guard.minute_asleep.entry(minute).or_insert(0);
                *min += 1;
            }
        }
    }

    // update current_guard to one with highest total sleep minutes
    let mut max_sleep: u64 = 0;
    for (id, guard) in &guards {
        if guard.sleep_total > max_sleep {
            max_sleep = guard.sleep_total;
            current_guard = *id;
        }
    }

    // minute most slept
    let mut max_minute: u64 = 0;
    let mut max_total: u64 = 0;
    let guard = guards.get(&current_guard).unwrap();
    for (minute, total) in &guard.minute_asleep {
        if *total > max_total {
            max_total = *total;
            max_minute = *minute;
        }
    }
    println!("guard: {}, total sleep: {}", current_guard, max_sleep);
    println!("minute: {}, total: {}", max_minute, max_total);
    println!("answer: {}", current_guard * max_minute);

    Ok(())
}
