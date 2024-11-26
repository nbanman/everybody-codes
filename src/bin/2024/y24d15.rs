use std::{borrow::Borrow, collections::{HashMap, HashSet, VecDeque}, ops::RangeInclusive};

use everybody_codes::{coord::Coord, indexer::Indexer, inputs::get_inputs, stopwatch::{ReportDuration, Stopwatch}};
use itertools::Itertools;
use lazy_regex::regex;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 15);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    // println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(forest: &str) -> usize {
    let width = forest.find(|c| c == '\n').unwrap() + 1;
    let height = (forest.len() + 1) / width;
    let start = forest.find(|c| c == '.').unwrap();
    let col_gates: Vec<usize> = regex!(r"\w..\w").find_iter(forest)
        .map(|m| m.start() + 2)
        .collect();
    let col_gate_widths: Vec<usize> = col_gates.iter()
        .map(|&gate| gate % width)
        .collect();
    let columns = vec![
        (0..=col_gate_widths[0]), 
        (col_gate_widths[0]..=col_gate_widths[1]), 
        (col_gate_widths[1]..=width - 2)
    ];
    println!("gates: {:?}, areas: {:?}", col_gates, columns);
    println!("forest len: {}", forest.len());
    let rows: Vec<RangeInclusive<usize>> = forest.lines().enumerate()
        .filter(|(_, line)| {
            line.as_bytes().iter().filter(|&&c| c == b'#').count() > width * 10 / 12
        })
        .map(|(idx, _)| idx)
        .tuple_windows()
        .map(|(top, bot)| top..=bot)
        .collect();
    println!("rows: {:?}", rows);
    let mut edges = HashMap::new();
    for ((col, x_bound), (row, y_bound)) in columns.iter()
        .enumerate()
        .cartesian_product(rows.iter().enumerate()) {
        let top_left = y_bound.start() * width + x_bound.start() + 1; // +1 needed to remove "fake" gates in liminal areas
        let bot_left = y_bound.end() * width + x_bound.start() + 1;
        let top_gates: HashSet<usize> = (top_left..top_left + x_bound.count() - 2) // -2 needed to remove "fake" gates in liminal areas
            .filter(|&idx| forest.as_bytes()[idx] == b'.')
            .collect(); 
        let bot_gates: HashSet<usize> = (bot_left..bot_left + x_bound.count() - 2)
            .filter(|&idx| forest.as_bytes()[idx] == b'.')
            .collect(); 

        let bot_gates = if bot_gates.is_empty() {
            let mut bot_gates = HashSet::new();
            if col < 2 { bot_gates.insert(col_gates[0]); }
            if col > 0 { bot_gates.insert(col_gates[1]); }
            bot_gates
        } else { 
            bot_gates
        };
        
        let start_gates = if col == 1 {
            &top_gates
        } else {
            &bot_gates
        };
        let end_gates = if col == 1 {
            &bot_gates
        } else {
            &top_gates
        };

        // determine what kind of partition we are in to set the state up properly
        let gate_status = if row == 0 {
            if col == 1 {
                GateStatus::One
            } else {
                GateStatus::Visited
            }
        } else if row == 4 && col == 1 {
            GateStatus::Two(None)
        } else {
            GateStatus::One
        };

        let forest = forest.as_bytes();

        for start_gate in start_gates {
            let mut q = VecDeque::new();
            let state = State {
                pos: *start_gate,
                herb: false,
                gate_status,
                return_gate: None,
            };
            q.push_back((0, state));
            let mut visited = HashSet::new();
            while let Some((steps, State {  
                pos,
                herb, 
                gate_status, 
                return_gate
            })) = q.pop_front() {
                let steps = steps + 1;
                let mut herb = herb;
                let mut gate_status = gate_status;
                let mut return_gate = return_gate;
                if end_gates.contains(&pos) {
                    match gate_status {
                        GateStatus::One => {
                            gate_status = GateStatus::Visited;
                            
                        },
                        GateStatus::Two(None) => GateStatus::Two(Some(pos)),
                        GateStatus::Two(Some(gate1)) => {
                            if pos == gate1 { 
                                continue; 
                            } else {
                                GateStatus::Visited
                            }
                        },
                        GateStatus::Visited => { continue; },
                    };

                    // somewhere in here I need to do the warp thing where a bunch of neighbors for
                    // each end_gate are created. I think I should make them here and then continue

                    continue;
                } else if start_gates.contains(&pos) {
                    unimplemented!();
                    // this is where i would start adding edges to the cache
                    continue
                }
                
                if forest[pos].is_ascii_alphabetic() && !herb {
                    herb = true;
                }

                // make normal neighbors here 
        }

    }
        
    3
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: usize,
    herb: bool,
    gate_status: GateStatus,
    return_gate: Option<usize>, // strongly consider rolling this info up into a GateStatus enum variant
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GateStatus {
    Visited,
    Two(Option<usize>),
    One,
}



#[allow(unused)]
fn to_coord(pos: usize, width: usize) -> Coord<usize, 2> {
    let x = pos % width;
    let y = pos / width;
    Coord::new2d(x, y)
}

#[allow(unused)]
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

#[allow(unused)]
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