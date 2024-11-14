use everybody_codes::{inputs::get_input, stopwatch::{ReportDuration, Stopwatch}};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    // let (input1, input2, input3) = get_inputs(24, 7);
    let input1 = get_input(24, 7, 1);
    // let input2 = get_input(24, X, 2);
    // let input3 = get_input(24, X, 3);
    println!("Inputs loaded ({})", stopwatch.lap().report());
    println!("1. {} ({})", solve(&input1), stopwatch.lap().report());
    // println!("2. {} ({})", solve(&input2), stopwatch.lap().report());
    // println!("3. {} ({})", solve(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: &str) -> usize {

}