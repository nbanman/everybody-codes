use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Default, Debug)]
pub struct Indexer<T: Hash + Eq + Clone> {
    id: usize,
    index_to_value: HashMap<usize, T>,
    value_to_index: HashMap<T, usize>,
}

impl <T: Hash + Eq + Clone> Indexer<T> {
    pub fn new() -> Self {
        let index_to_value: HashMap<usize, T> = HashMap::new();
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
                    self.index_to_value.insert(value_id, value.to_owned());
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
            self.index_to_value.insert(value_id, value.to_owned());
            Some(value_id)
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.value_to_index.contains_key(value)
    }

    pub fn value(self, index: usize) -> Option<T> {
        self.index_to_value.get(&index).cloned()
    }

    pub fn remove_by_index(&mut self, index: usize) -> Option<T> {
        let removal = self.index_to_value.remove(&index);
        if let Some(value) = removal {
            self.value_to_index.remove(&value);
            Some(value)
        } else {
            None    
        }
    }

    pub fn remove_by_value(&mut self, value: T) -> Option<usize> {
        let removal = self.value_to_index.remove(&value);
        if let Some(index) = removal {
            self.index_to_value.remove(&index);
            Some(index)
        } else {
            None    
        }
    }
}
