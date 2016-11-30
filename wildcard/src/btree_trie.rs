use std::collections::{BTreeMap,btree_map};
use protocoll::MapMut;
use trie::Trie;

#[derive(Default,Debug,Clone,PartialEq,Eq)]
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

    fn insert<I>(&mut self, mut s:I) where I:Iterator<Item = char> {
        match s.next() {
            None => self.accept = true,
            Some(c) => self.node.update_mut(c, BTreeMapTrie::new(), |t| t.insert(s))
        }
    }
}

impl Trie for BTreeMapTrie {
    fn learn<I>(mut self, s:I) -> Self where I:Iterator<Item = char> {
        self.insert(s);
        self
    }

    // fn learn<I>(mut self, mut s:I) -> Self where I:Iterator<Item = char> {
    //     match s.next() {
    //         None => {self.accept = true; self}
    //         Some(c) => BTreeMapTrie {
    //             node:self.node.update(c, |opt_t| opt_t.unwrap_or_default().learn(s)),
    //             accept:self.accept
    //         }
    //     }
    // }

    fn recognize<I>(&self, mut s:I) -> bool where I:Iterator<Item = char> {
        match s.next() {
            None => self.accept,
            Some(c) => match self.node.get(&c) {
                None => false,
                Some(t) => t.recognize(s)
            }
        }
    }

    fn prefix_search<'a,I>(&'a self, s:I) -> Box<Iterator<Item = String> + 'a>
        where I:Iterator<Item = char> {
        let mut p = String::new();
        let mut t = self;
        for c in s {
            p.push(c);
            t = match t.node.get(&c) {
                Some(t) => t,
                None => return Box::new(Vec::new().into_iter())
            }
        }
        let init = if t.accept {vec![p.to_owned()]} else {Vec::new()};
        Box::new(init.into_iter().chain(Iter{stack:vec![t.node.iter()],prefix:p}))
    }
}

#[derive(Clone)]
pub struct Iter<'a> {
    stack:Vec<btree_map::Iter<'a,char,BTreeMapTrie>>,
    prefix:String
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut end = match self.stack.pop() {
            None => return None,
            Some(end) => end
        };
        match end.next() {
            None => {
                self.prefix.pop();
self.next()
            },
            Some((&c,t)) => {
                self.prefix.push(c);
                self.stack.push(end);
                self.stack.push(t.node.iter());
                if t.accept {Some(self.prefix.to_owned())} else {self.next()}
            }
        }
    }
}

use heapsize::HeapSizeOf;
impl HeapSizeOf for BTreeMapTrie {
    fn heap_size_of_children(&self) -> usize {
        self.node.heap_size_of_children()
    }
}
