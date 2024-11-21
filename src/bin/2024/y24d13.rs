use std::{cmp::{min, Reverse}, collections::{BinaryHeap, HashSet}};

use everybody_codes::{cardinals::Cardinal, inputs::get_inputs, stopwatch::{ReportDuration, Stopwatch}};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pub time: usize,
    pub pos: usize,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 13);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input1), stopwatch.lap().report());
    println!("2. {} ({})", solve(&input2), stopwatch.lap().report());
    println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(chamber: &str) -> usize {
    let width = chamber.find('\n').unwrap() + 1;
    let start_pos = chamber.find('E').unwrap();
    let chamber = chamber.as_bytes();

    // start Dijkstra
    let mut q: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut visited = HashSet::new();
    let start = State { time: 0, pos: start_pos };
    q.push(Reverse(start));
    let moves = [Cardinal::North, Cardinal::South, Cardinal::East, Cardinal::West];
    while let Some(Reverse(state)) = q.pop() {
        if chamber[state.pos] == b'S' { return state.time; }
        if !visited.insert(state.pos) { continue; }
        moves.iter()
            .filter_map(|dir| {
                let neighbor_pos= match dir {
                    Cardinal::North => state.pos.checked_sub(width),
                    Cardinal::East => Some(state.pos + 1),
                    Cardinal::South => Some(state.pos + width),
                    Cardinal::West => Some(state.pos - 1),
                };
                let Some(neighbor_pos) = neighbor_pos else {
                    return None;
                };
                let neighbor_value = if chamber[neighbor_pos] == b'S' {
                    48
                } else {
                    chamber[neighbor_pos]
                };
                let Some(neighbor_value) = (neighbor_value as char).to_digit(10) else { 
                    return None; 
                };
                let current_value = chamber[state.pos];
                let current_value = if current_value == b'E' { 0 } else { current_value - 48 };
                if visited.contains(&neighbor_pos) { return None; }
                let time_cost = (1 + min(
                    (neighbor_value as i8 - current_value as i8).rem_euclid(10),
                    (current_value as i8 + 10 - neighbor_value as i8).rem_euclid(10)
                )) as usize;
                Some(State { time: state.time + time_cost, pos: neighbor_pos }) 
            })
            .for_each(|next| q.push(Reverse(next)));
    }
    unreachable!("Queue empty, but S never reached!");
}

#[test]
fn tests() {
    let tests = ["#######
#6769##
S50505E
#97434#
#######", "SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS"];
    assert_eq!(28, solve(tests[0]));
    assert_eq!(14, solve(tests[1]));
    // assert_eq!(X, solve(tests[2]));
}