use protocoll::{Map,Str,Seq};
use protocoll::map::VecSortedMap;

pub trait Trie {
    fn learn<I>(self, s:I) -> Self where I:Iterator<Item = char>;
    fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char>;
    fn prefix_search<I>(&self, s:I) -> Vec<String> where I:Iterator<Item = char>;
}

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct ArrayMapTrie {
    pub arrmap:VecSortedMap<char,ArrayMapTrie>,
    pub accept:bool
}

impl Default for ArrayMapTrie {
    fn default() -> Self {
        ArrayMapTrie::new()
    }
}

impl ArrayMapTrie {
    pub fn new() -> Self {
        ArrayMapTrie{arrmap:VecSortedMap::new(),accept:false}
    }

    fn _prefix_search(&self, prefix:String) -> Vec<String> {
        self.arrmap.iter().fold
            (if self.accept {Vec::new().inc(prefix.clone())} else {Vec::new()},
             |ret,&(c,ref t)| ret.plus(t._prefix_search(prefix.clone().inc(c))))
    }
}

impl Trie for ArrayMapTrie {
    fn learn<I>(mut self, mut s:I) -> Self where I:Iterator<Item = char> {
        match s.next() {
            None => {self.accept = true; self}
            Some(c) => ArrayMapTrie {
                arrmap:self.arrmap.update(c, |opt_t| opt_t.unwrap_or_default().learn(s)).shrink(),
                accept:self.accept
            }
        }
    }

    fn recognize<I>(&self, mut s:I) -> bool where I:Iterator<Item = char> {
        match s.next() {
            None => self.accept,
            Some(c) => match self.arrmap.get(&c) {
                None => false,
                Some(t) => t.recognize(s)
            }
        }
    }

    fn prefix_search<I>(&self, mut s:I) -> Vec<String> where I:Iterator<Item = char> {
        let mut p = String::from("");
        let mut t = self;
        loop {
            match s.next() {
                None => return t._prefix_search(p),
                Some(c) => match t.arrmap.get(&c) {
                    None => return Vec::new(),
                    Some(tc) => {p.push(c); t = tc}
                }
            }
        }
    }
}


// #[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
// pub enum ArrayMapTrie {
//     Accept(VecSortedMap<char,ArrayMapTrie>),
//     NonAcc(VecSortedMap<char,ArrayMapTrie>)
// }

// impl Default for ArrayMapTrie {
//     fn default() -> Self {
//         ArrayMapTrie::new()
//     }
// }

// impl ArrayMapTrie {
//     pub fn new() -> Self {
//         ArrayMapTrie::NonAcc(VecSortedMap::new())
//     }

//     pub fn is_accept(&self) -> bool {
//         match self {
//             &ArrayMapTrie::Accept(_) => true,
//             &ArrayMapTrie::NonAcc(_) => false
//         }
//     }

//     pub fn view_content(&self) -> &VecSortedMap<char,ArrayMapTrie> {
//         match self {
//             &ArrayMapTrie::Accept(ref m) => m,
//             &ArrayMapTrie::NonAcc(ref m) => m
//         }
//     }

//     fn _prefix_search(&self, prefix:String) -> Vec<String> {
//         self.view_content().iter().fold
//             (if self.is_accept() {Vec::new().inc(prefix.clone())} else {Vec::new()},
//              |ret,&(c,ref t)| ret.plus(t._prefix_search(prefix.clone().inc(c))))
//     }
// }

// impl Trie for ArrayMapTrie {
//     fn learn<I>(self, mut s:I) -> Self where I:Iterator<Item = char> {
//         match (self,s.next()) {
//             (ArrayMapTrie::Accept(m),None) => ArrayMapTrie::Accept(m),
//             (ArrayMapTrie::NonAcc(m),None) => ArrayMapTrie::Accept(m),
//             (ArrayMapTrie::Accept(m),Some(c)) => ArrayMapTrie::Accept
//                 (m.update(c, |opt_t| opt_t.unwrap_or_default().learn(s))),
//             (ArrayMapTrie::NonAcc(m),Some(c)) => ArrayMapTrie::NonAcc
//                 (m.update(c, |opt_t| opt_t.unwrap_or_default().learn(s)))
//         }
//     }

//     fn recognize<I>(&self, mut s:I) -> bool where I:Iterator<Item = char> {
//         match s.next() {
//             None => self.is_accept(),
//             Some(c) => match self.view_content().get(&c) {
//                 None => false,
//                 Some(t) => t.recognize(s)
//             }
//         }
//     }

//     fn prefix_search<I>(&self, mut s:I) -> Vec<String> where I:Iterator<Item = char> {
//         let mut p = String::from("");
//         let mut t = self;
//         loop {
//             match s.next() {
//                 None => return t._prefix_search(p),
//                 Some(c) => match t.view_content().get(&c) {
//                     None => return Vec::new(),
//                     Some(tc) => {p.push(c); t = tc}
//                 }
//             }
//         }
//     }
// }
