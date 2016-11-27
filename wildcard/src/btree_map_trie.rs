use std::collections::BTreeMap;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum BTreeMapTrie {
    Accept(BTreeMap<char,BTreeMapTrie>),
    NonAcc(BTreeMap<char,BTreeMapTrie>)
}

impl Default for BTreeMapTrie {
    fn default() -> Self {
        BTreeMapTrie::new()
    }
}

impl BTreeMapTrie {
    pub fn new() -> Self {
        BTreeMapTrie::NonAcc(BTreeMap::new())
    }

    pub fn is_accept(&self) -> bool {
        match self {
            &BTreeMapTrie::Accept(_) => true,
            &BTreeMapTrie::NonAcc(_) => false
        }
    }

    pub fn view_content(&self) -> &BTreeMap<char,BTreeMapTrie> {
        match self {
            &BTreeMapTrie::Accept(ref m) => m,
            &BTreeMapTrie::NonAcc(ref m) => m
        }
    }

    fn _learn<I>(self, mut s:I) -> Self where I:Iterator<Item = char> {
        match (self,s.next()) {
            (BTreeMapTrie::Accept(m),None) => BTreeMapTrie::Accept(m),
            (BTreeMapTrie::NonAcc(m),None) => BTreeMapTrie::Accept(m),
            (BTreeMapTrie::Accept(m),Some(c)) => BTreeMapTrie::Accept
                (m.update(c, |opt_t| opt_t.unwrap_or_default()._learn(s))),
            (BTreeMapTrie::NonAcc(m),Some(c)) => BTreeMapTrie::NonAcc
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

impl Trie for BTreeMapTrie {
    fn learn(self, s:&str) -> Self {
        self._learn(s.chars())
    }

    fn recognize(&self, s:&str) -> bool {
        self._recognize(s.chars())
    }

    fn prefix_search(&self, prefix:&str) -> Vec<String> {
        let mut s = prefix.chars();
        let mut t = self;
        loop {
            match s.next() {
                None => return t._prefix_search(String::from(prefix)),
                Some(c) => match t.view_content().get(&c) {
                    None => return Vec::new(),
                    Some(tc) => t = tc
                }
            }
        }
    }
}
