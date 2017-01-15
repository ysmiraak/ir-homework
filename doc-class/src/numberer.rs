// Author: Kuan Yu, 3913893
// Honor Code:  I pledge that this program represents my own work.

use std::collections::HashMap;

/// injective mapping: str -> usize
pub trait Numberer {
    fn number(&mut self, s: &str) -> usize;
}

#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct HashMapNumberer(HashMap<String, usize>);

impl HashMapNumberer {
    pub fn new() -> Self {
        HashMapNumberer(HashMap::new())
    }
}

impl Numberer for HashMapNumberer {
    fn number(&mut self, s: &str) -> usize {
        match self.0.get(s) {
            Some(&n) => n,
            None => {
                let n = self.0.len();
                self.0.insert(s.to_owned(), n);
                n
            }
        }
    }
}
