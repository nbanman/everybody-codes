use std::{cmp::{max, min}, collections::{HashMap, HashSet, VecDeque}, ops::RangeInclusive};

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

fn solve(input: &str) -> usize {
    let forest = Forest::new(input).expect("Invalid input!");
    println!("columns: {:?}, rows: {:?}", forest.columns, forest.rows);
    let graph: HashMap<Vertex, Vec<Edge>> = get_graph(&forest);
    3
}

fn get_graph(forest: &Forest<'_>) -> HashMap<Vertex, Vec<Edge>> {
    let adjacent = vec![-(forest.width as isize), 1, forest.width as isize, -1];

    for ((col, x_bound), (row, y_bound)) in forest.columns.iter()
        .enumerate()
        .cartesian_product(forest.rows.iter().enumerate()) {
        
        let (top_gates, bot_gates) = get_gates(forest, x_bound, y_bound, col);
        println!("col: {}, row: {}, top gates: {:?}, bot gates: {:?}", col, row, top_gates, bot_gates);
    }
    HashMap::new()
}

fn get_gates(
    forest: &Forest<'_>,
    x_bound: &RangeInclusive<usize>,
    y_bound: &RangeInclusive<usize>,
    col: usize,
) -> (HashSet<usize>, HashSet<usize>) {
    let top_left = y_bound.start() * forest.width + x_bound.start() + 1; // +1 needed to remove "fake" gates in liminal areas
    let bot_left = y_bound.end() * forest.width + x_bound.start() + 1;
    let top_gates: HashSet<usize> = (top_left..top_left + x_bound.clone().count() - 2) // -2 needed to remove "fake" gates in liminal areas
        .filter(|&idx| forest.features.as_bytes()[idx] == b'.')
        .collect(); 
    let bot_gates: HashSet<usize> = (bot_left..bot_left + &x_bound.clone().count() - 2)
        .filter(|&idx| forest.features.as_bytes()[idx] == b'.')
        .collect(); 

    let bot_gates = if bot_gates.is_empty() {
        let mut bot_gates = HashSet::new();
        if col < 2 { bot_gates.insert(forest.bottom_gates[0]); }
        if col > 0 { bot_gates.insert(forest.bottom_gates[1]); }
        bot_gates
    } else { 
        bot_gates
    };
    (top_gates, bot_gates)
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pub go_from: usize,
    pub return_to: usize,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    pub weight: usize,
    pub go_to: usize,
    pub return_from: usize,
}

#[derive(Debug)]
struct Forest<'a> {
    pub features: &'a str,
    pub width: usize,
    pub start: usize,
    pub bottom_gates: Vec<usize>,
    pub columns: Vec<RangeInclusive<usize>>, 
    pub rows: Vec<RangeInclusive<usize>>,
}

impl<'a> Forest<'a> {
    pub fn new(features: &'a str) -> Option<Self> {
        let width = features.find(|c| c == '\n')? + 1;
        let start = features.find(|c| c == '.')?;

        // columns. Hard-coded as three
        let bottom_gates: Vec<usize> = regex!(r"\w..\w").find_iter(features)
            .map(|m| m.start() + 2)
            .collect();
        let bottom_gate_widths: Vec<usize> = bottom_gates.iter()
            .map(|&gate| gate % width)
            .collect();
        let columns: Vec<RangeInclusive<usize>> = vec![
            (0..=bottom_gate_widths[0]), 
            (bottom_gate_widths[0]..=bottom_gate_widths[1]), 
            (bottom_gate_widths[1]..=width - 2)
        ];

        // find "rows," which are delimited by narrow openings following an herb

        // first find what lines the herbs are
        let lines: Vec<(usize, &str)> = features.lines().enumerate().collect();

        let herb_lines: Vec<usize> = lines.iter()
            .filter(|(_, line)| line.as_bytes().iter().any(|c| c.is_ascii_alphabetic()))
            .map(|(idx, _)| *idx)
            .collect();

        let delimiters: Vec<usize> = herb_lines.iter()
            .map(|herb_line| {
                lines[*herb_line + 1..].iter().find(|(_, line)| {
                    line.as_bytes().iter().filter(|&&c| c == b'#').count() > width * 10 / 12
                }).unwrap().0
            })
            .dedup()
            .collect();

        let mut rows: Vec<usize> = Vec::with_capacity(delimiters.len() + 2);
        rows.push(0);
        rows.extend_from_slice(&delimiters);

        let rows: Vec<RangeInclusive<usize>> = rows.into_iter()
            .tuple_windows()
            .map(|(top, bot)| top..=bot)
            .collect();

        Some(Self { features, width, start, bottom_gates, columns, rows, })
    }
}

#[allow(unused)]
fn old_solve(forest: &str) -> usize {
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
    let adjacent = vec![-(width as isize), 1, width as isize, -1];
    for ((col, x_bound), (row, y_bound)) in columns.iter()
        .enumerate()
        .cartesian_product(rows.iter().enumerate()) {
        let top_left = y_bound.start() * width + x_bound.start() + 1; // +1 needed to remove "fake" gates in liminal areas
        let bot_left = y_bound.end() * width + x_bound.start() + 1;
        let top_gates: HashSet<usize> = (top_left..top_left + x_bound.clone().count() - 2) // -2 needed to remove "fake" gates in liminal areas
            .filter(|&idx| forest.as_bytes()[idx] == b'.')
            .collect(); 
        let bot_gates: HashSet<usize> = (bot_left..bot_left + &x_bound.clone().count() - 2)
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
                GateStatus::Zero
            }
        } else if row == 4 && col == 1 {
            GateStatus::Two(None)
        } else {
            GateStatus::One
        };

        let forest = forest.as_bytes();

        let mut visited = HashMap::new();

        for start_gate in start_gates {
            let mut q = VecDeque::new();
            let state = State {
                pos: *start_gate,
                herb: false,
                gate_status,
            };
            q.push_back((0, state));
            let mut herb_edges_remaining = start_gates.len() * max(1, end_gates.len());

            while let Some((steps, state)) = q.pop_front() {                
                visited.insert((*start_gate, state), steps);
                if end_gates.contains(&state.pos) {
                    if state.herb || row == 3 {
                        herb_edges_remaining -= 1;
                        if herb_edges_remaining == 0 {
                            break;
                        }
                    }
                } else if state.gate_status == GateStatus::Zero && forest[state.pos].is_ascii_alphabetic() {
                    herb_edges_remaining -= 1;
                    if herb_edges_remaining == 0 {
                        break;
                    }
                }

                adjacent.iter()
                    .filter_map(|&adj| {
                        let neighbor_pos = usize::try_from((state.pos as isize) + adj).ok()?;
                        if forest[neighbor_pos] == b'#' { return None; }
                        let neighbor_gate_status = if end_gates.contains(&neighbor_pos) {
                            if let GateStatus::Two(None) = state.gate_status {
                                GateStatus::Two(Some(neighbor_pos))
                            } else {
                                state.gate_status
                            }
                        } else {
                            state.gate_status
                        };
                        let neighbor_herb = if !state.herb && forest[neighbor_pos].is_ascii_alphabetic() {
                            true
                        } else {
                            state.herb
                        };
                        let neighbor_state = State {
                            pos: neighbor_pos,
                            herb: neighbor_herb,
                            gate_status: neighbor_gate_status,
                        };
                        if visited.contains_key(&(*start_gate, neighbor_state)) {
                            None
                        } else {
                            Some(neighbor_state)
                        }
                    })
                    .for_each(|neighbor_state| {
                        q.push_back((steps + 1, neighbor_state));
                    });
            }
        }
         // make edges
         let paths: HashMap<(usize, Option<usize>), Vec<((usize, State), i32)>> = visited.into_iter()
            .filter(|&((_, State { pos, herb: _, gate_status: _ }), _)| {
                end_gates.contains(&pos) || (row == 0 && col != 1 && forest[pos].is_ascii_alphabetic())
            })
            // .sorted_unstable_by_key(|&((start_gate, state), _)| (start_gate, state.pos))  
            .into_group_map_by(|&((start_gate, state), _)| {
                if row == 0 && col != 1 {
                    (start_gate, None)
                } else {
                    (start_gate, Some(state.pos))
                }
            });

        if row == 0 && col != 1 {
            let combos = start_gates.iter().cartesian_product(start_gates.iter());
            for (&go_from, &return_from) in combos {
                let steps = paths[&(go_from, None)].first().unwrap().1 
                    + paths[&(return_from, None)].first().unwrap().1;
                // need to know from, to, return, steps
                edges.entry((go_from, return_from))
                    .or_insert(Vec::new())
                    .push((steps, None));
            }

        } else {
            let combos = start_gates.iter()
                .cartesian_product(end_gates.iter())
                .cartesian_product(end_gates.iter())
                .cartesian_product(start_gates.iter());
            for (((&go_from, &go_to), &return_from), &return_to) in combos {
                let paths_go = paths[&(go_from, Some(go_to))].clone();
                let (paths_go_short, paths_go_long) = paths_go.iter()
                    .sorted_unstable_by_key(|(_, steps)| steps)
                    .collect_tuple()
                    .unwrap();
                let paths_return = paths[&(return_to, Some(return_from))].clone();
                let (paths_return_short, paths_return_long) = paths_return.iter()
                    .sorted_unstable_by_key(|(_, steps)| steps)
                    .collect_tuple()
                    .unwrap();
                let steps = min(paths_go_short.1 + paths_return_long.1, paths_go_long.1 + paths_return_short.1);
                // need to know from, to, return, steps
                edges.entry((go_from, return_from))
                    .or_insert(Vec::new())
                    .push((steps, Some(go_to)));
            }
        }
    }
    3
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    pos: usize,
    herb: bool,
    gate_status: GateStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum GateStatus {
    Zero,
    One,
    Two(Option<usize>),
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