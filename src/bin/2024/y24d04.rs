use std::usize;

use everybody_codes::inputs::get_inputs;
use itertools::Itertools;

fn main() {
    let (input1, input2, input3) = get_inputs(24, 4);
    println!("1. {}", lowest(&input1));
    println!("2. {}", lowest(&input2));
    println!("3. {}", least(&input3));
}

fn lowest(input: &str) -> usize {
    let nails: Vec<usize> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let &smallest = nails.iter().min().unwrap();
    nails.into_iter()
        .map(|nail| nail - smallest)
        .sum()
}

fn least(input: &str) -> usize {
    let nails: Vec<usize> = input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .sorted()
        .collect();
    let target = nails[nails.len() / 2];
    
    nails.into_iter()
        .map(|nail| target.abs_diff(nail))
        .sum()
}


#[test]
fn examples() {
    let test1 = r"3
4
7
8";
     let test3 = r"2
4
5
6
8";
    assert_eq!(10, lowest(&test1));
    assert_eq!(8, least(&test3));
}
