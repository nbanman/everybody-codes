use std::iter::successors;

use everybody_codes::{inputs::get_inputs, stopwatch::{ReportDuration, Stopwatch}, indexer::Indexer};
use itertools::Itertools;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_inputs(24, 11);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", get_population(&input1, 4, "A"), stopwatch.lap().report());
    println!("2. {} ({})", get_population(&input2, 10, "Z"), stopwatch.lap().report());
    println!("3. {} ({})", minmax_population(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn get_population(input: &str, days: usize, start: &str) -> usize {
    let (generations, start_key) = get_generations(input, start);
    let mut population = vec![0usize; generations.len()];
    population[start_key] = 1;
    breed(population, &generations, days)
}

fn minmax_population(input: &str) -> usize {
    let (generations, _) = get_generations(input, "None");
    let (min, max) = (0..generations.len())
        .map(|termite| {
            let mut population = vec![0usize; generations.len()];
            population[termite] = 1;
            breed(population, &generations, 20)
        })
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

fn get_generations(input: &str, start: &str) -> (Vec<Vec<usize>>, usize) {
    let mut indexer = Indexer::new();
    let generations: Vec<_> = input.lines()
        .map(|line| {
            let (prev, next) = line.split_once(':').unwrap();
            let id = indexer.get_or_assign(&prev);
            let children: Vec<_> = next.split(',')
                .map(|child| indexer.get_or_assign(&child))
                .collect();
            (id, children)
        })
        .sorted_unstable()
        .map(|(_, children)| children)
        .collect();
    (generations, indexer.get_or_assign(&start))
}

fn breed(population: Vec<usize>, generations: &[Vec<usize>], days: usize) -> usize {
    let next_gen = |pop: &[usize]| {
        let mut next_gen = vec![0; pop.len()];
        for (termite, &amt) in pop.iter().enumerate() {
            let offspring = generations.get(termite).unwrap();
            for &child in offspring {
                next_gen[child] += amt;
            }
        }
        next_gen
    };
    successors(Some(population), |pop| Some(next_gen(pop)))
        .take(days + 1)
        .last()
        .unwrap()
        .into_iter()
        .sum()
}

#[test]
fn tests() {
    let tests = ["A:B,C
B:C,A
C:A", "A:B,C
B:C,A,A
C:A"];
    assert_eq!(8, get_population(tests[0], 4, "A"));
    assert_eq!(268815, minmax_population(tests[1]));
}