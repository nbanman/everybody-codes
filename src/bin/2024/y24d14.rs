use everybody_codes::{cardinals::Cardinal, coord::Coord3, inputs::get_input, stopwatch::{ReportDuration, Stopwatch}};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    // let (input1, input2, input3) = get_inputs(24, 14);
    let input1 = get_input(24, 14, 1);
    // let input2 = get_input(24, 14, 2);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", solve(&input2), stopwatch.lap().report());
    // println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: &str) -> usize {
    let positions: Vec<i64> = input   
        .split(',')
        .scan((Cardinal::North, Coord3::origin()), |(dir, pos), instruction| {
            let (turn, length) = instruction.split_at(1);
            let turn = turn.as_bytes()[0] as char;
            let distance: i64 = length.parse().unwrap();
            *dir = match turn {
                'U' => Cardinal::South,
                'D' => Cardinal::North,
                'R' => Cardinal::East,
                'L' => Cardinal::West,
                'F' => *dir,
                'B' => dir.flip(),
                other => panic!("'{other}' not recognized instruction.")
            };
            *pos = pos.move_direction(*dir, distance).unwrap();
            Some((*dir, *pos)) 
        })
        .map(|(_, pos)| pos.y())
        .collect();
    for p in positions {
        println!("{p}");
    }
    3
}

#[test]
fn tests() {
    let tests = ["U5,R3,D2,L5,U4,R5,D2"];
    assert_eq!(7, solve(tests[0]));
    // assert_eq!(X, solve(tests[1]));
    // assert_eq!(X, solve(tests[2]));
}