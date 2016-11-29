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

    fn search_add(&self, p:&str, mut acc:Vec<String>) -> Vec<String> {
        let n = match self.0 {
            None => return acc,
            Some(ref n) => n
        };
        acc = n.lo.search_add(p,acc);
        acc = n.hi.search_add(p,acc);
        let mut p = p.to_owned();
        p.push(n.c);
        acc = n.eq.search_add(&p,acc);
        if n.accept {acc.push(p);}
        acc
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
            let ret = Vec::new();
        let p:String = s.collect();
        let (t,add_p) = match self.search(p.chars()) {
            (None,_) => return Box::new(ret.into_iter()),
            (Some(t),add_p) => (t,add_p)
        };
        let mut ret = t.search_add(&p,ret);
        if add_p {ret.push(p);}
        Box::new(ret.into_iter())
    }
}

// #[derive(Clone)]
// pub struct Iter {
//     stack:Vec<TernaryTrie>,
//     prefix:String,
//     cons_prefix:bool
// }

// impl<'a> Iterator for Iter<'a> {
//     type Item = String;
//     fn next(&mut self) -> Option<String> {
//         if self.cons_prefix {
//             self.cons_prefix = false;
//             return Some(self.prefix.to_owned())
//         }
//         let mut end = match self.stack.pop() {
//             None => return None,
//             Some(i) => i
//         };
//         match end.next() {
//             None => {
//                 self.prefix.pop(); 
//                 self.next()
//             },
//             Some(&(c, ref t)) => {
//                 self.prefix.push(c);
//                 self.stack.push(end);
//                 self.stack.push(t.node.iter());
//                 match t.accept {
//                     true => Some(self.prefix.to_owned()),
//                     false => self.next()
//                 }
//             }
//         }
//     }
// }

// impl<'a> IntoIterator for &'a ArrayMapTrie {
//     type Item = String;
//     type IntoIter = Iter<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }   

// }
