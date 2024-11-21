use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use everybody_codes::{cardinals::Cardinal, inputs::get_input, stopwatch::{ReportDuration, Stopwatch}};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pub time: usize,
    pub pos: usize,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    // let (input1, input2, input3) = get_inputs(24, X);
    let input1 = get_input(24, X, 1);
    // let input2 = get_input(24, X, 2);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", solve(&input2), stopwatch.lap().report());
    // println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(chamber: &str) -> usize {
    let width = chamber.find('\n').unwrap() + 1;
    let start_pos = chamber.find('S').unwrap();
    let end_pos = chamber.find('E').unwrap();
    let chamber = chamber.as_bytes();

    // start Dijkstra
    let mut q: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut visited = HashSet::new();
    let start = State { time: 0, pos: start_pos };
    q.push(Reverse(start));
    let moves = [Cardinal::North, Cardinal::South, Cardinal::East];
    while let Some(Reverse(state)) = q.pop() {
        if !visited.insert(state.pos) { continue; }
        let floor = (chamber[state.pos] as char).to_digit(10).unwrap() as i8;
        moves.iter()
            .filter_map(|dir| {
                let neighbor_pos= match dir {
                    Cardinal::North => state.pos.checked_sub(width),
                    Cardinal::East => Some(state.pos + 1),
                    Cardinal::South => Some(state.pos + width),
                    Cardinal::West => panic!("Should not be able to go West"),
                };
                let Some(neighbor_pos) = neighbor_pos else {
                    return None;
                };
                let Some(neighbor_value) = 
                    (chamber[neighbor_pos] as char).to_digit(10) else {
                        return None; 
                    };
                let neighbor_value = neighbor_value as i8;
                if visited.contains(&neighbor_pos) { return None; }
                let time_cost = 1 + min(
                    (neighbor_value - 
                );
                None 
            });
    }
    3
}

#[test]
fn tests() {
    let tests = ["#######
#6769##
S50505E
#97434#
#######"];
    assert_eq!(X, solve(tests[0]));
    // assert_eq!(X, solve(tests[1]));
    // assert_eq!(X, solve(tests[2]));
}