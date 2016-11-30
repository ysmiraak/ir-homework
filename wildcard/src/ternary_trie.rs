use std::cmp::Ordering::{Less,Equal,Greater};
use trie::Trie;

#[derive(Default,Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct TernaryTrie(Option<Box<TernaryTrieNode>>);

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct TernaryTrieNode {
    c:char,
    lo:TernaryTrie,
    eq:TernaryTrie,
    hi:TernaryTrie,
    accept:bool
}

impl TernaryTrieNode {
    fn new(c:char) -> Self {
        TernaryTrieNode {
            c:c,
            lo:TernaryTrie::new(),
            eq:TernaryTrie::new(),
            hi:TernaryTrie::new(),
            accept:false
        }
    }
}

impl TernaryTrie {
    pub fn new() -> Self {
        TernaryTrie(None)
    }

    fn learn_char<I>(self, c:char, mut s:I) -> Self where I:Iterator<Item = char> {
        let mut n = match self.0 {
            None => TernaryTrieNode::new(c),
            Some(n) => *n
        };
        match c.cmp(&n.c) {
            Less => n.lo = n.lo.learn_char(c,s),
            Equal => match s.next() {
                Some(c) => n.eq = n.eq.learn_char(c,s),
                None => n.accept = true},
            Greater => n.hi = n.hi.learn_char(c,s)
        };
        TernaryTrie(Some(Box::new(n)))
    }


    fn search_char(&self, c:char) -> (Option<&Self>,bool) {
        let n = match self.0 {
            None => return (None,false),
            Some(ref n) => n
        };
        match c.cmp(&n.c) {
            Less => n.lo.search_char(c),
            Equal => (Some(&n.eq),n.accept),
            Greater => n.hi.search_char(c)
        }
    }

    fn search<I>(&self, s:I) -> (Option<&Self>,bool) where I:Iterator<Item = char> {
        let mut ret = false;
        let mut t = self;
        for c in s {
            let (opt_t,accept) = t.search_char(c);
            t = match opt_t {
                None => return (None,false),
                Some(t) => t};
            ret = accept
        }
        (Some(t),ret)
    }
}

use std::fmt::{Display,Formatter,Result};
impl Display for TernaryTrie {
    fn fmt(&self, f:&mut Formatter) -> Result {
        match self.0 {
            None => write!(f, "_"),
            Some(ref n) => {
                let (l,r) = if n.accept {('[',']')} else {('(',')')};
                write!(f,"{}{} {} {} {}{}",l,n.c,n.lo,n.eq,n.hi,r)
            }
        }
    }
}

impl Trie for TernaryTrie {
    fn learn<I>(self, mut s:I) -> Self where I:Iterator<Item = char> {
        match s.next() {
            Some(c) => self.learn_char(c,s),
            None => self
        }
    }

    fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char> {
        self.search(s).1
    }

    fn prefix_search<'a,I>(&'a self, s:I) -> Box<Iterator<Item = String> + 'a>
        where I:Iterator<Item = char> {
        let p:String = s.collect();
        let (n,add_p) = match self.search(p.chars()) {
            (None,_) => return Box::new(Vec::new().into_iter()),
            (Some(t),add_p) => match t.0 {
                None => return Box::new(Vec::new().into_iter()),
                Some(ref n) => (n,add_p)
            }
        };
        let init = if add_p {vec![p.to_owned()]} else {Vec::new()};
        Box::new(init.into_iter().chain(Iter{stack:vec![(n,0)],prefix:p}))
    }
}

#[derive(Clone)]
pub struct Iter<'a> {
    stack:Vec<(&'a TernaryTrieNode,u8)>,
    prefix:String
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let (end,act) = match self.stack.pop() {
            None => return None,
            Some(end) => end
        };
        let (next,opt_ret) = match act {
            0 => (&end.lo,None),
            1 => {
                self.prefix.push(end.c);
                let opt_ret = if end.accept {Some(self.prefix.to_owned())} else {None};
                (&end.eq,opt_ret)
            }
            2 => {
                self.prefix.pop();
                (&end.hi,None)
            }
            _ => return self.next()
        };
        self.stack.push((end,act+1));
        if let Some(ref n) = next.0 {self.stack.push((n,0))}
        match opt_ret {
            Some(s) => Some(s),
            None => self.next()
        }
    }
}

use heapsize::HeapSizeOf;
impl HeapSizeOf for TernaryTrieNode {
    fn heap_size_of_children(&self) -> usize {
        self.lo.heap_size_of_children()
            + self.eq.heap_size_of_children()
            + self.hi.heap_size_of_children()
    }
}

impl HeapSizeOf for TernaryTrie {
    fn heap_size_of_children(&self) -> usize {
        self.0.heap_size_of_children()
    }
}
