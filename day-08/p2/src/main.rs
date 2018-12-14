use failure::Error;
use indextree::{Arena, Node, NodeId};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Entry {
    childs: u64,
    metadata: u64,
    entries: Vec<u64>,
}

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

fn create_graph(input: &[u64], arena: &mut Arena<Entry>) {
    let mut childs = Vec::<u64>::new();
    let mut data = Vec::<u64>::new();
    let mut i = 0_usize;
    let mut parent_node = NodeId::new(999);
    loop {
        let x = input[i];

        if x == 0 {
            let child_node = arena.new_node(Entry {
                childs: x,
                metadata: input[i + 1],
                entries: Vec::<u64>::new(),
            });
            let node = arena.get_mut(child_node).unwrap();
            i += 1;
            for _ in 0..input[i] {
                i += 1;
                node.data.entries.push(input[i]);
            }
            parent_node.append(child_node, arena).unwrap();

            while let Some(last) = childs.last_mut() {
                *last -= 1;
                if *last == 0 {
                    childs.pop().unwrap();
                    if let Some(last_data) = data.last() {
                        let pnode = arena.get_mut(parent_node).unwrap();
                        for _ in 0..*last_data {
                            i += 1;
                            pnode.data.entries.push(input[i]);
                        }
                        data.pop();
                        if let Some(pid) = pnode.parent() {
                            parent_node = pid;
                        }
                    }
                } else {
                    break;
                }
            }
        } else {
            let node = arena.new_node(Entry {
                childs: input[i],
                metadata: input[i + 1],
                entries: Vec::<u64>::new(),
            });

            if arena.get(parent_node).is_some() {
                parent_node.append(node, arena).unwrap();
            }

            parent_node = node;

            childs.push(input[i]);
            i += 1;
            data.push(input[i]);
        }
        i += 1;

        if i == input.len() {
            break;
        }
    }
}

// recursively sum total
fn calculate_total(node: &Node<Entry>, tree: &Arena<Entry>) -> u64 {
    let mut total = 0_u64;

    if node.data.childs == 0 {
        return node.data.entries.iter().sum::<u64>();
    } else {
        for n in &node.data.entries {
            if let Some(child) = node.first_child() {
                let mut child_node = tree.get(child).unwrap();
                if *n == 1 {
                    total += calculate_total(child_node, tree);
                } else {
                    let mut siblings = Vec::<NodeId>::new();
                    siblings.push(child);
                    while let Some(sibling) = child_node.next_sibling() {
                        siblings.push(sibling);
                        child_node = tree.get(sibling).unwrap();
                    }
                    if *n <= siblings.len() as u64 {
                        let sibling_node = tree.get(siblings[*n as usize - 1]).unwrap();
                        total += calculate_total(sibling_node, tree);
                    }
                }
            } else {
                return 0;
            };
        }
    }
    total
}

fn main() -> Result<(), Error> {
    let input = parse_input("input.txt")?;
    let tree = &mut Arena::<Entry>::new();
    create_graph(&input, tree);
    let root_node = tree.get(NodeId::new(0)).unwrap();
    let answer = calculate_total(root_node, tree);
    println!("answer: {}", answer);
    Ok(())
}
