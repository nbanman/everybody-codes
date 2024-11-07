use everybody_codes::inputs::get_inputs;
use itertools::Itertools;

fn main() {
    let (input1, input2, input3) = get_inputs(24, 1);
    println!("1. {}", solve(&input1, 1));
    println!("2. {}", solve(&input2, 2));
    println!("3. {}", solve(&input3, 3));
}

fn solve(input: &str, group_size: usize) -> usize {
    input
        .as_bytes()
        .into_iter()
        .chunks(group_size)
        .into_iter()
        .map(|baddies| {
            let baddies: Vec<u8> = baddies.cloned().collect();
            let number_of_baddies = baddies
                .iter()
                .filter(|&&it| it != b'x')
                .count();
            // println!("baddies: {:?}, #: {number_of_baddies}", baddies);
            let potions = |baddie: &u8| -> usize {
                match &baddie {
                    b'A' => 0,
                    b'B' => 1,
                    b'C' => 3,
                    b'D' => 5,
                    _ => 0,
                }
            };
            let bonus_potions = number_of_baddies * 
                (number_of_baddies.checked_sub(1).unwrap_or_default());
            bonus_potions + (baddies.iter().map(|baddie| potions(baddie)).sum::<usize>())
        }).sum()
}

#[test]
fn default() {
    let (input1, input2, input3) = get_inputs(24, 1);
    assert_eq!(1354, solve(&input1, 1));
    assert_eq!(5639, solve(&input2, 2));
    assert_eq!(28180, solve(&input3, 3));
}