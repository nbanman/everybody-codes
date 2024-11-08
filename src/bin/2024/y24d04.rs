use std::usize;

use everybody_codes::inputs::get_inputs;
use itertools::Itertools;

fn main() {
    let (input1, input2, input3) = get_inputs(24, 4);
    println!("1. {}", solve(&input1, lowest));
    println!("2. {}", solve(&input2, lowest));
    println!("3. {}", solve(&input3, least));
}

fn lowest(nails: &Vec<usize>) -> usize {
    *nails.iter().min().unwrap()
}

fn least(nails: &Vec<usize>) -> usize {
    let nails: Vec<_> = nails.iter().sorted().collect();
    *nails[nails.len() / 2]
}

fn solve<F>(input: &str, get_target: F) -> usize 
where
    F: FnOnce(&Vec<usize>) -> usize,
{
    let nails: Vec<usize> = input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    let target = get_target(&nails);
    
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
    assert_eq!(10, solve(&test1, lowest));
    assert_eq!(8, solve(&test3, least));
}
