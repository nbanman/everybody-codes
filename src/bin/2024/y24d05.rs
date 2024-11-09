use std::{cmp::min, collections::{HashSet, VecDeque}};

use everybody_codes::{get_numbers::ContainsNumbers, inputs::get_inputs};

fn main() {
    let (input1, input2, input3) = get_inputs(24, 5);
    println!("1. {}", part1(&input1));
    println!("2. {}", part2(&input2));
    println!("3. {}", part3(&input3));
}

fn parse_input(input: &str) -> (usize, Vec<VecDeque<usize>>) {
    let number_of_columns = 1 + input.as_bytes().iter()
        .take_while(|&&c| c != b'\n')
        .filter(|&&c| c == b' ')
        .count(); 
    let mut columns = vec![VecDeque::new(); number_of_columns];
    for (idx, n) in input.get_numbers::<usize>().enumerate() {
        columns[idx % number_of_columns].push_back(n);
    }
    (number_of_columns, columns)
}

fn play_round(round: usize, number_of_columns: usize, columns: &mut Vec<VecDeque<usize>>) -> String {
    let clapper_col = (round - 1) % number_of_columns;
    let next_col = round % number_of_columns;
    let next_len = columns[next_col].len();
    let clapper = columns[clapper_col].pop_front().unwrap();  
    let pos = (clapper - 1) % (next_len * 2);
    let pos = min(pos, next_len) - pos.checked_sub(next_len).unwrap_or_default();
    
    columns[next_col].insert(pos, clapper);
    
    columns.iter()
        .map(|column| { 
            let number = *column.front().unwrap();
            number.to_string()
        }).collect::<String>()
}

fn part1(input: &str) -> usize {
    let (number_of_columns, mut columns) = parse_input(input);
    for round in 1..10 {
        play_round(round, number_of_columns, &mut columns);
    }
    play_round(10, number_of_columns, &mut columns)
        .parse()
        .unwrap()
}


fn part2(input: &str) -> usize {
    let (number_of_columns, mut columns) = parse_input(input);
    let digits = input.lines().next().unwrap().chars()
        .filter(|&c| c.is_ascii_digit())
        .count();
    let mut counter = vec![0usize; 10usize.pow(digits as u32)];
    for round in 1.. {
        let shouted: usize = play_round(round, number_of_columns, &mut columns).parse().unwrap();
        counter[shouted] += 1;
        if counter[shouted] == 2024 {
            return round * shouted
        }
    }
    unreachable!()
}

fn part3(input: &str) -> usize {
    let (number_of_columns, mut columns) = parse_input(input);
    let mut cache = HashSet::new();
    let mut highest_number = 0;
    for round in 1.. {
        let shouted: usize = play_round(round, number_of_columns, &mut columns).parse().unwrap();
        let state: String = columns.iter()
            .flat_map(|column| {
                column.iter().map(|n| n.to_string())
            }).collect();
        if !cache.insert(state) {
            return highest_number;
        }
        if highest_number < shouted { highest_number = shouted; }
    }
    unreachable!()
}

#[test]
fn examples() {
    let test1 = r"
2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4
    ".trim();
    let test2 = r"
2 3 4 5
6 7 8 9
    ".trim();
    assert_eq!(2323, part1(test1, 10));
    assert_eq!(50877075, part2(test2));
    assert_eq!(6584, part3(test2));
}
