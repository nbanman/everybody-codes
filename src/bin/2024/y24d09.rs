use std::{cmp::min, usize};

use everybody_codes::{get_numbers::ContainsNumbers, inputs::get_input, stopwatch::{ReportDuration, Stopwatch}};
use itertools::Itertools;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    // let (input1, input2, input3) = get_inputs(24, 9);
    let input1 = get_input(24, 9, 1);
    // let input2 = get_input(24, 9, 2);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    let stamps = [10, 5, 3, 1];
    println!("1. {} ({})", solve(&input1, &stamps), stopwatch.lap().report());
    // println!("2. {} ({})", solve(&input2), stopwatch.lap().report());
    // println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: &str, stamps: &[usize]) -> usize {
    let stamps: Vec<usize> = stamps.iter().cloned().sorted_unstable().rev().collect();
    let brightnesses: Vec<usize> = input.get_numbers().sorted().rev().collect();
    let mut cache: Vec<Option<usize>> = vec![None; brightnesses[0] + 1];
    for stamp in &stamps {
        cache[*stamp] = Some(1);
    }
    let answer = brightnesses.iter()
        .map(|brightness| {
            if let Some(beetles) = cache[*brightness] {
                beetles
            } else {
                println!("\tFinding {brightness}");
                let beetles = trav(*brightness, usize::MAX, &mut cache, &stamps);
                println!("\t{brightness} takes {beetles} beetles");
                beetles
            }
        })
        .sum();
    println!("{answer}");
    answer
}

fn trav (
    remaining: usize,
    prev_best: usize,
    cache: &mut Vec<Option<usize>>,
    stamps: &[usize],
) -> usize {
    // cache hit; return precomputed answer
    if let Some(count) = cache[remaining] {
        return count;
    }

    // otherwise, carry on
    let mut best = usize::MAX;

    // iterate through each stamp in descending order
    for (idx, &stamp) in stamps.iter().enumerate() {
        let beetles = remaining / stamp;

        // if the best possible outcome requries more beetles than prev_best, abort
        if beetles >= prev_best {
            return prev_best;
        }

        if beetles >= best {
            return best;
        }

        // if divides cleanly, we have our answer, update cache and pop up with it
        if remaining % stamp == 0 { // may not need first conditional
            if cache[remaining] == None {
                cache[remaining] = Some(beetles);
            }
            return beetles;
        }

        // otherwise try decreasing amounts of that stamp
        // only runs if the stamp is smaller than the remaining brightness
        for n in (1..=beetles).rev() {  
            // this should be the best obtainable with n number of stamp
            if best <= n { continue; }
            let result = n + trav(
                remaining - n * stamp,
                best - n, // the -n is an experiment
                cache,
                &stamps[idx..],
            );
            if result < best { 
                best = result; 
            }
        }
    }
    cache[remaining] = Some(best);
    best
}

#[test]
fn tests() {
    let tests = [r"2, 4, 7, 16", "27"];
    let stamps = [[1, 3, 5, 10], [1, 3, 9, 10]];
    assert_eq!(3, solve(tests[1], &stamps[1]));
    assert_eq!(10, solve(tests[0], &stamps[0]));
    // assert_eq!(9, part2(tests[1]));
    // assert_eq!(9, part3(tests[2]));
}

