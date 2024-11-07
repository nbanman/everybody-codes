use std::{collections::HashSet, iter::successors};

use everybody_codes::{coord::Coord2, include_input};

fn main() {
    let input1 = include_input!(24 / 03 / 1);
    println!("1. {}", solve(input1, false));
    let input2 = include_input!(24 / 03 / 2);
    println!("2. {}", solve(input2, false));
    let input3 = include_input!(24 / 03 / 3);
    println!("3. {}", solve(input3, true));
}

fn solve(input: &str, diagonals: bool) -> usize {
    let width = input.find(|it| it == '\n').unwrap() + 1;
    let blocks: HashSet<Coord2> = input.as_bytes().into_iter()
        .enumerate()
        .filter(|(_, &c)| c == b'#') 
        .map(|(idx, _)| {
            let x = (idx % width) as i64;
            let y = (idx / width) as i64;
            Coord2::new2d(x, y)
        }).collect();
    let dig = |blocks: &HashSet<Coord2>| -> HashSet<Coord2> {
        blocks.iter()
            .filter(|block| { 
                block.adjacent(diagonals).iter()
                    .all(|pos| blocks.contains(pos))
            }).cloned()
            .collect()
    };
    let stages = successors(Some(blocks), |blocks| {
        let next = dig(blocks);
        if next.is_empty() {
            None
        } else {
            Some(next)
        }
    });
    stages
        .fold(0, |count, stage| {
            count + stage.len()
        })

}

#[test]
fn default() {
    let input1 = include_input!(24 / 03 / 1);
    assert_eq!(134, solve(input1, false));
    let input2 = include_input!(24 / 03 / 2);
    assert_eq!(2810, solve(input2, false));
    let input3 = include_input!(24 / 03 / 3);
    assert_eq!(10443, solve(input3, true));
}