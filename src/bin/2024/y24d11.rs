use std::{collections::HashMap, iter::successors};

use everybody_codes::{inputs::get_inputs, stopwatch::{ReportDuration, Stopwatch}};
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
    let generations = get_generations(input);
    let mut population = HashMap::new();
    population.insert(start.to_string(), 1);
    breed(population, &generations, days)
}

fn minmax_population(input: &str) -> usize {
    let generations = get_generations(input);
    let termites: Vec<String> = generations.keys().cloned().collect();
    let (min, max) = termites.into_iter()
        .map(|termite| {
            let mut population = HashMap::new();
            population.insert(termite, 1);
            breed(population, &generations, 20)
        })
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

fn get_generations(input: &str) -> HashMap<String, Vec<String>> {
    input.lines()
        .map(|line| {
            let (prev, next) = line.split_once(':').unwrap();
            let next = next.split(',').map(|s| s.to_string()).collect();
            (prev.to_string(), next)
        })
        .collect()
}

fn next_gen(
    pop: &HashMap<String, usize>, 
    generations: &HashMap<String, Vec<String>>,
) -> HashMap<String, usize> 
{
    let mut next_gen = HashMap::new();
    for (termite, &amt) in pop.iter() {
        let offspring = generations.get(termite).unwrap();
        for child in offspring {
            next_gen.entry(child.clone())
                .and_modify(|n| *n += amt)
                .or_insert(amt);
        }
    }
    next_gen
}

fn breed(population: HashMap<String, usize>, generations: &HashMap<String, Vec<String>>, days: usize) -> usize {
    successors(Some(population), |pop| Some(next_gen(pop, &generations)))
        .take(days + 1)
        .last()
        .unwrap()
        .values()
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