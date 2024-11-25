use std::collections::{HashSet, VecDeque};

use everybody_codes::{indexer::Indexer, inputs::get_inputs, stopwatch::{ReportDuration, Stopwatch}};
use itertools::Itertools;
use lazy_regex::regex;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 15);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(forest: &str) -> usize {
    let width = forest.find(|c| c == '\n').unwrap() + 1;
    let start = forest.find(|c| c == '.').unwrap();
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let forest = forest.as_bytes();
    let adjacent = vec![-(width as isize), 1, width as isize, -1];

    q.push_back((0, start));
    while let Some((steps, pos)) = q.pop_front() {
        if !visited.insert(pos) { continue; }
        if forest[pos] == b'H' { 
            return steps * 2
        }
        adjacent.iter()
            .filter_map(|&adj| usize::try_from((pos as isize) + adj).ok() )
            .filter(|neighbor| {
                !visited.contains(neighbor) && forest[*neighbor] != b'#'
            })
            .map(|neighbor| (steps + 1, neighbor))
            .for_each(|neighbor_state| q.push_back(neighbor_state));
    }
    unreachable!("Queue drained before herb found!");
}

fn part2(forest: &str) -> usize {
    let width = forest.find(|c| c == '\n').unwrap() + 1;
    let start = forest.find(|c| c == '.').unwrap();
    let herbs_in_forest: HashSet<char> = forest.chars()
        .filter(char::is_ascii_alphabetic)
        .collect();
    let mut herb_indexer = Indexer::new();
    for herb in herbs_in_forest.iter() {
        herb_indexer.assign(*herb);
    }
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let forest = forest.as_bytes();
    let adjacent = vec![-(width as isize), 1, width as isize, -1];
    
    q.push_back((0, start, vec![false; herb_indexer.len()]));
    while let Some((steps, pos, herbs)) = q.pop_front() {
        if !visited.insert((pos, herbs.clone())) { continue; }
        if herbs.iter().all(|&herb| herb) && pos == start {
            return steps
        }
        adjacent.iter()
            .filter_map(|&adj| usize::try_from((pos as isize) + adj).ok() )
            .filter(|neighbor| {
                !visited.contains(&(*neighbor, herbs.clone())) && forest[*neighbor] != b'#' && forest[*neighbor] != b'~'
            })
            .map(|neighbor| {
                let mut neighbor_herbs = herbs.clone();
                let terrain = forest[neighbor] as char;
                if terrain.is_ascii_alphabetic() {
                    let herb_index = herb_indexer.get_index(&terrain).unwrap();
                    if !neighbor_herbs[herb_index] {
                        // println!("{terrain} reached in {} steps, pos: {}", steps + 1, Coord::new2d(neighbor % width, neighbor / width));
                        neighbor_herbs[herb_index] = true;
                    }
                }
                (steps + 1, neighbor, neighbor_herbs) 
            })
            .for_each(|neighbor_state| q.push_back(neighbor_state));
    }
    
    unreachable!();
}

fn solve(forest: &str) -> usize {
    let width = forest.find(|c| c == '\n').unwrap() + 1;
    let start = forest.find(|c| c == '.').unwrap();
    let gates: Vec<usize> = regex!(r"\w..\w").find_iter(forest)
        .flat_map(|m| [m.start() + 1, m.start() + 2])
        .collect();

    let herbs_in_forest: Vec<HashSet<char>> = forest.chars().enumerate()
        .filter(|(_, c)| c.is_ascii_alphabetic())
        .chunk_by(|(idx, _)| (width - 1) / (idx % width))
        .into_iter()
        .map(|(_, group)| group.map(|(_, c)| c).collect())
        .collect();
        
    let mut herb_indexer = Indexer::new();
    for herb in herbs_in_forest.iter() {
        herb_indexer.assign(*herb);
    }
    3
}

#[test]
fn tests() {
    let tests = ["#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########", "##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################", ""];
    assert_eq!(26, part1(tests[0]));
    assert_eq!(38, part2(tests[1]));
    // assert_eq!(X, solve(tests[2]));
}