use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn get_inputs(year: u16, day: u8) -> (String, String, String) {
    let year = if year > 2000 { year - 2000 } else { year };
    let path = format!("./inputs/everybody_codes_e20{}_q{}_p", year, day);
    (1..=3).into_iter()
        .map(|part| {
            let mut path = path.clone();
            path.push_str(&part.to_string());
            path.push_str(".txt");
            let mut file = File::open(&path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        }).collect_tuple()
        .unwrap()
}