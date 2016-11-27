use std::collections::BTreeMap;
use protocoll::{Map,Str,Seq};
use trie::Trie;

#[derive(Default,Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct BTreeMapTrie {
    node:BTreeMap<char,BTreeMapTrie>,
    accept:bool
}

impl BTreeMapTrie {
    pub fn new() -> Self {
        BTreeMapTrie {
            node:BTreeMap::new(),
            accept:false
        }
    }

    fn search_prefixed(&self, p:String) -> Vec<String> {
        self.node.iter().fold
            (if self.accept {Vec::new().inc(p.to_owned())} else {Vec::new()},
             |ret,(&c,t)| ret.plus(t.search_prefixed(p.to_owned().inc(c))))
    }
}

impl Trie for BTreeMapTrie {
    fn learn<I>(mut self, mut s:I) -> Self where I:Iterator<Item = char> {
        match s.next() {
            None => {self.accept = true; self}
            Some(c) => BTreeMapTrie {
                node:self.node.update(c, |opt_t| opt_t.unwrap_or_default().learn(s)),
                accept:self.accept
            }
        }
    }

    fn recognize<I>(&self, mut s:I) -> bool where I:Iterator<Item = char> {
        match s.next() {
            None => self.accept,
            Some(c) => match self.node.get(&c) {
                None => false,
                Some(t) => t.recognize(s)
            }
        }
    }

    fn prefix_search<I>(&self, s:I) -> Vec<String> where I:Iterator<Item = char> {
        let p:String = s.collect();
        let mut t = self;
        for c in p.chars() {
            t = match t.node.get(&c) {
                None => return Vec::new(),
                Some(t) => t
            }
        }
        t.search_prefixed(p)
    }
}
