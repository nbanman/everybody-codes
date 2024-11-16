use std::collections::HashSet;

use everybody_codes::{inputs::get_input, stopwatch::{ReportDuration, Stopwatch}};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    // let (input1, input2, input3) = get_inputs(24, 10);
    let input1 = get_input(24, 10, 1);
    let input2 = get_input(24, 10, 2);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    // println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    // println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn part1(input: &str) -> String {
    let (hz, vt, length) = crosstabs(input);
    get_runes(length, &hz, &vt)
}

fn get_runes(length: usize, hz: &Vec<HashSet<char>>, vt: &Vec<HashSet<char>>) -> String {
    let mut runes = String::new();
    for y in 0..length {
        for x in 0..length {
            let &cross = hz[y].intersection(&vt[x]).next().unwrap();
            runes.push(cross);
        }
    }
    runes
}

fn part2(input: &str) -> usize {
    let samples: Vec<String> = get_samples(input);
    samples.iter() 
        .map(|sample| {
            let (hz, vt, length) = crosstabs(sample);
            let runes = get_runes(length, &hz, &vt);
            runes.as_bytes().into_iter().enumerate()
                .map(|(idx, &rune)| (rune as usize - 64) * (idx + 1))
                .sum::<usize>()
        })
        .sum()
}

fn get_samples(input: &str) -> Vec<String> {
    let width = input.find('\n').unwrap() + 1;
    let rows: Vec<&str> = input.split("\n\n").collect();
    let samples_per_row = width / 9;
    rows.into_iter()
        .flat_map(|row| get_row_samples(row, samples_per_row))
        .collect()
}

fn get_row_samples(row: &str, samples_per_row: usize) -> Vec<String> {
    let mut samples = vec![String::new(); samples_per_row];
    for line in row.lines() {
        for (place, c) in line.chars().enumerate() {
            let sample_idx = place / 9;
            let place = place % 9;
            let sample = &mut samples[sample_idx];
            let insert = if place < 8 { c } else { '\n' };
            sample.push(insert);
        }
        samples.last_mut().unwrap().push('\n');
    }
    samples
}


fn crosstabs(sample: &str) -> (Vec<HashSet<char>>, Vec<HashSet<char>>, usize) {
    println!("called\n{sample}\n");
    let length = sample.find(|c| c != '*').unwrap() * 2;
    let mut hz = vec![HashSet::new(); length];
    let mut vt = vec![HashSet::new(); length];
    let mut hz_index = 0;
    for line in sample.lines() {
        if line.starts_with('*') {
            line.chars()
            .filter(|c| c.is_alphabetic())
            .enumerate()
            .for_each(|(idx, c)| { vt[idx].insert(c); });
        } else {
            line.chars().for_each(|c| { hz[hz_index].insert(c); });
            hz_index += 1;
        }
    }
    (hz, vt, length)
}

#[test]
fn tests() {
    let inputs = ["**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**"];
    assert_eq!(10, part1(inputs[0]));
    // assert_eq!(10, solve(tests[1]));
    // assert_eq!(10, solve(tests[2]));
}