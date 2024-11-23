use std::{collections::HashMap, hash::Hash};

use crate::coord::Coord2;

#[derive(Clone, Default, Debug)]
pub struct Indexer<T: Hash + Eq + Clone> {
    id: usize,
    index_to_value: Vec<T>,
    value_to_index: HashMap<T, usize>,
}

impl <T: Hash + Eq + Clone> Indexer<T> {
    pub fn new() -> Self {
        let index_to_value: Vec<T> = Vec::new();
        let value_to_index: HashMap<T, usize> = HashMap::new();
        Self {
            id: 0,
            index_to_value,
            value_to_index,
        }
    }

    pub fn get_or_assign(&mut self, value: &T) -> usize {
        *self.value_to_index.entry(value.clone())
                .or_insert_with(|| {
                    let value_id = self.id;
                    self.id += 1;
                    self.index_to_value.push(value.to_owned());
                    value_id
                })
    }

    pub fn get(&self, value: &T) -> Option<usize> {
        self.value_to_index.get(value).copied()
    }

    pub fn assign(&mut self, value: &T) -> Option<usize> {
        if self.value_to_index.contains_key(value) {
            None
        } else {
            let value_id = self.id;
            self.id += 1;
            self.value_to_index.insert(value.to_owned(), value_id);
            self.index_to_value.push(value.to_owned());
            Some(value_id)
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.value_to_index.contains_key(value)
    }

    pub fn value(&self, index: usize) -> Option<T> {
        self.index_to_value.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.index_to_value.len()
    }
}

#[test]
fn basic_functionality() {
    let mut indexer = Indexer::new();
    let one_one = Coord2::new2d(1, 1);
    let three_three = Coord2::new2d(3, 3);
    assert_eq!(Some(0), indexer.assign(&one_one));
    assert_eq!(None, indexer.assign(&one_one));
    assert_eq!(Some(1), indexer.assign(&Coord2::origin()));
    assert_eq!(true, indexer.contains(&one_one));
    assert_eq!(false, indexer.contains(&three_three));
    assert_eq!(Some(one_one), indexer.value(0));
    assert_eq!(None, indexer.value(2));
    assert_eq!(1, indexer.get_or_assign(&Coord2::origin()));
    assert_eq!(2, indexer.get_or_assign(&three_three));
}