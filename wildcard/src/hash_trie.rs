use std::collections::HashMap;
use protocoll::{Map,Str,Seq};
use trie::Trie;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum HashMapTrie {
    Accept(HashMap<char,HashMapTrie>),
    NonAcc(HashMap<char,HashMapTrie>)
}

impl Default for HashMapTrie {
    fn default() -> Self {
        HashMapTrie::new()
    }
}

impl HashMapTrie {
    pub fn new() -> Self {
        HashMapTrie::NonAcc(HashMap::new())
    }

    pub fn is_accept(&self) -> bool {
        match self {
            &HashMapTrie::Accept(_) => true,
            &HashMapTrie::NonAcc(_) => false
        }
    }

    pub fn view_content(&self) -> &HashMap<char,HashMapTrie> {
        match self {
            &HashMapTrie::Accept(ref m) => m,
            &HashMapTrie::NonAcc(ref m) => m
        }
    }

    fn _learn<I>(self, mut s:I) -> Self where I:Iterator<Item = char> {
        match (self,s.next()) {
            (HashMapTrie::Accept(m),None) => HashMapTrie::Accept(m),
            (HashMapTrie::NonAcc(m),None) => HashMapTrie::Accept(m),
            (HashMapTrie::Accept(m),Some(c)) => HashMapTrie::Accept
                (m.update(c, |opt_t| opt_t.unwrap_or_default()._learn(s))),
            (HashMapTrie::NonAcc(m),Some(c)) => HashMapTrie::NonAcc
                (m.update(c, |opt_t| opt_t.unwrap_or_default()._learn(s)))
        }
    }

    fn _recognize<I>(&self, mut s:I) -> bool where I:Iterator<Item = char> {
        match s.next() {
            None => self.is_accept(),
            Some(c) => match self.view_content().get(&c) {
                None => false,
                Some(t) => t._recognize(s)
            }
        }
    }

    fn _prefix_search(&self, prefix:String) -> Vec<String> {
        self.view_content().iter()
            .fold(if self.is_accept() {Vec::new().inc(prefix.clone())} else {Vec::new()},
                  |ret,(c,t)| ret.plus(t._prefix_search(prefix.clone().inc(*c))))
    }
}

impl Trie for HashMapTrie {
    fn learn<I>(self, s:I) -> Self where I:Iterator<Item = char> {
        self._learn(s)
    }

    fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char> {
        self._recognize(s)
    }

    fn prefix_search<I>(&self, mut s:I) -> Vec<String> where I:Iterator<Item = char> {
        let mut p = String::new();
        let mut t = self;
        loop {
            match s.next() {
                None => return t._prefix_search(p),
                Some(c) => {
                    p.push(c);
                    match t.view_content().get(&c) {
                        None => return Vec::new(),
                        Some(tc) => t = tc
                    }
                }
            }
        }
    }
}
