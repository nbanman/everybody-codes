use std::collections::HashMap;
use everybody_codes::inputs::get_inputs;
use itertools::Itertools;

type Branches<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let (input1, input2, input3) = get_inputs(24, 6);
    println!("1. {}", solve(&input1, false));
    println!("2. {}", solve(&input2, true));
    println!("3. {}", solve(&input3, true));
}

fn solve(input: &str, truncate: bool) -> String {
    let branches = get_branches(input);
    let paths = get_paths(truncate, branches);
    get_strongest(paths)
}

fn get_branches(input: &str) -> Branches {
    input.lines()
        .filter(|line| {
            let possible_pest = &line[0..3];
            possible_pest != "ANT" && possible_pest != "BUG"
        })
        .map(|line| {
            let (parent, children) = line.split_once(':').unwrap();
            let children: Vec<&str> = children.split(',').collect();
            (parent, children)
        })
        .collect()
}

fn get_paths(truncate: bool, branches: HashMap<&str, Vec<&str>>) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    
    let mut q = vec![vec!["RR"]];
    while let Some(path) = q.pop() {
        let &current = path.last().unwrap();
        if current == "@" {
            let mut path_name = String::new();
        
            for s in path {
                if truncate {
                    path_name.push(s.chars().next().unwrap());
                } else {
                    path_name.push_str(s);
                }
            }
            paths.push(path_name);
        } else {
            if let Some(children) = branches.get(current) {
                for child in children {
                    // if path.contains(child) { continue; }
                    let mut new_path = path.clone();
                    new_path.push(child);
                    q.push(new_path);
                }
            } 
        }
    }
    paths
}

fn get_strongest(paths: Vec<String>) -> String {
    paths.iter()
        .into_group_map_by(|&s| s.len())
        .iter()
        .find(|(_, paths)| paths.len() == 1)
        .unwrap()
        .1[0]
        .to_string()
}

#[test]
fn examples() {
    let test1 = r"
RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@
    ".trim();
    assert_eq!("RRB@".to_string(), solve(test1, false));
    assert_eq!("RB@", solve(test1, true));
}
