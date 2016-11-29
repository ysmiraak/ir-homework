extern crate protocoll;

mod array_trie;
mod btree_trie;
mod hash_trie;
mod ternary_trie;

pub mod trie {
    pub trait Trie {
        fn learn<I>(self, s:I) -> Self where I:Iterator<Item = char>;
        fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char>;
        fn prefix_search<'a,I>(&'a self, s:I) -> Box<Iterator<Item = String> + 'a>
            where I:Iterator<Item = char>;
    }

    pub use array_trie::ArrayMapTrie;
    pub use btree_trie::BTreeMapTrie;
    pub use hash_trie::HashMapTrie;
    pub use ternary_trie::TernaryTrie;
}

pub mod query {
    use trie::Trie;

    /// iterates all strings with prefix `p` in trie `t`.
    pub fn prefix_query<'a,T>(t:&'a T, p:&str) ->
        Box<Iterator<Item = String> + 'a>
        where T:Trie {
        t.prefix_search(p.chars())
    }

    /// iterates all strings with suffix `s` in reverse trie `r`.
    pub fn suffix_query<'a,T>(r:&'a T, s:&str) ->
        Box<Iterator<Item = String> + 'a>
        where T:Trie {
        Box::new(r.prefix_search(s.chars().rev()).map(|s| s.chars().rev().collect()))
    }

    /// iterates all strings with prefix `s` and suffix `p` in either trie `t` or
    /// reverse trie `r`, depending on which one is more efficient.
    pub fn circumfix_query<'a,T>(t:&'a T, r:&'a T, p:&'a str, s:&'a str) ->
        Box<Iterator<Item = String> + 'a>
        where T:Trie {
        let len = p.len() + s.len();
        if 2 * p.len() <= s.len() {
            Box::new(prefix_query(t,p).filter(move |q| len <= q.len() && q.ends_with(s)))
        } else {
            Box::new(suffix_query(r,s).filter(move |q| len <= q.len() && q.starts_with(p)))
        }
    }

    /// iterates all strings in either trie `t` or reverse trie `r` that fit the
    /// query expression `q` which should contain one `*`.
    pub fn wildcard_query<'a,T>(t:&'a T, r:&'a T, q:&'a str) ->
        Result<Box<Iterator<Item = String> + 'a>, &'a str>
        where T:Trie {
        let ps:Vec<&str> = q.split('*').collect();
        if ps.len() != 2 {return Err("sorry, only queries with one * are supported.")}
        let (p,s) = (ps[0],ps[1]);
        match (ps[0].is_empty(),ps[1].is_empty()) {
            (true,true) => Err("you don't want to do this."),
            (false,true) => Ok(prefix_query(t,p)),
            (true,false) => Ok(suffix_query(r,s)),
            (false,false) => Ok(circumfix_query(t,r,p,s))
        }
    }
}
