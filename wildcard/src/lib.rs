extern crate protocoll;

mod array_trie;
mod btree_trie;
mod hash_trie;
mod ternary_trie;

pub mod trie {
    pub trait Trie {
        fn learn<I>(self, s:I) -> Self where I:Iterator<Item = char>;
        fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char>;
        fn prefix_search<I>(&self, s:I) -> Vec<String> where I:Iterator<Item = char>;
    }

    pub use array_trie::ArrayMapTrie;
    pub use btree_trie::BTreeMapTrie;
    pub use hash_trie::HashMapTrie;
    pub use ternary_trie::TernaryTrie;
}

pub mod query {
    use protocoll::set::VecSortedSet;
    use trie::Trie;
    
    pub fn prefix_query<T>(t:&T, p:&str) -> VecSortedSet<String> where T:Trie {
        t.prefix_search(p.chars()).into_iter().collect()
    }

    pub fn suffix_query<T>(t_rev:&T, s:&str) -> VecSortedSet<String> where T:Trie {
        t_rev.prefix_search(s.chars().rev()).into_iter()
            .map(|s| s.chars().rev().collect()).collect()
    }

    pub fn circumfix_query<T>(t:&T, t_rev:&T, p:&str, s:&str) -> VecSortedSet<String> where T:Trie {
        let min_len = p.len() + s.len();
        let s1:VecSortedSet<_> = prefix_query(t,p).into_iter()
            .filter(|s| s.len() >= min_len).collect();
        let s2:VecSortedSet<_> = suffix_query(t_rev,s).into_iter()
            .filter(|s| s.len() >= min_len).collect();
        s1 & s2
    }

    pub fn wildcard_query<'a,T>(t:&T, t_rev:&T, q:&str) -> Result<VecSortedSet<String>,&'a str>
        where T:Trie {
        let ps:Vec<&str> = q.split('*').collect();
        if ps.len() != 2 {return Err("sorry, only queries with one * are supported.")}
        match (ps[0].is_empty(),ps[1].is_empty()) {
            (true,true) => Err("you don't want to do this."),
            (false,true) => Ok(prefix_query(t,ps[0])),
            (true,false) => Ok(suffix_query(t_rev,ps[1])),
            (false,false) => Ok(circumfix_query(t,t_rev,ps[0],ps[1]))
        }
    }
}
