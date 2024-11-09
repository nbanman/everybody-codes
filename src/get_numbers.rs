use std::str::FromStr;
use num_traits::PrimInt;
use crate::try_get::TryGet;

pub fn get_numbers<N: PrimInt+FromStr>(s: &str) -> Vec<N> {
    let mut numbers: Vec<N> = Vec::new();
    let mut start_position = -1isize;
    let s = s.as_bytes();
    for (position, c) in s.iter().enumerate() {
        let position = position as isize;
        if c.is_ascii_digit() ||
            (c == &b'-' && s.try_get(position - 1).map(|x| x.is_ascii_digit()) != Some(true)) {
            if start_position == -1 {
                start_position = position
            }
        } else if start_position != -1 {
            if let Ok(sub_str) = std::str::from_utf8(&s[start_position as usize..position as usize]) {
                if let Ok(parsed) = sub_str.parse::<N>() {
                    numbers.push(parsed);
                }
            }
            start_position = -1;
        }
    }
    if start_position != -1 {
        if let Ok(sub_str) = std::str::from_utf8(&s[start_position as usize..]) {
            if let Ok(parsed) = sub_str.parse::<N>() {
                numbers.push(parsed);
            }
        }
    }
    numbers
}

pub struct NumberIterator<'a, N: PrimInt+FromStr> {
    s: &'a [u8],
    start_position: isize,
    position: isize,
    phantom: std::marker::PhantomData<N>,
}

impl<'a, N: PrimInt+FromStr> NumberIterator<'a, N> {
    pub fn new(input: &'a str) -> Self {
        NumberIterator {
            s: input.as_bytes(),
            start_position: -1,
            position: 0,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, N: PrimInt+FromStr> Iterator for NumberIterator<'a, N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.s.len() as isize {
            let c = self.s[self.position as usize];

            if c.is_ascii_digit() ||
                (c == b'-' && self.s.try_get(self.position - 1).map(|x| x.is_ascii_digit()) != Some(true)) {
                if self.start_position == -1 {
                    self.start_position = self.position;
                }
            } else if self.start_position != -1 {
                if let Ok(sub_str) =
                    std::str::from_utf8(&self.s[self.start_position as usize..self.position as usize]) {
                    if let Ok(parsed) = sub_str.parse::<N>() {
                        self.start_position = -1;
                        self.position += 1;
                        return Some(parsed);
                    } else {
                        self.start_position = -1;
                    }
                }
            }
            self.position += 1;
        }

        if self.start_position != -1 {
            if let Ok(sub_str) = std::str::from_utf8(&self.s[self.start_position as usize..]) {
                if let Ok(parsed) = sub_str.parse::<N>() {
                    self.start_position = -1; // probably unnecessary
                    return Some(parsed);
                }
            }
        }
        None
    }
}

pub trait ContainsNumbers {
    fn get_numbers<N: PrimInt+FromStr>(&self) -> NumberIterator<'_, N>;
}

impl<'a> ContainsNumbers for &'a str {
    fn get_numbers<N: PrimInt+FromStr>(&self) -> NumberIterator<'_, N> {
        NumberIterator::new(self)
    }
}