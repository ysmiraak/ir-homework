extern crate protocoll;

mod ternary_trie;
mod hash_map_trie;
mod btree_map_trie;
mod array_map_trie;

mod trie {
    pub trait Trie {
        fn learn<I>(self, s:I) -> Self where I:Iterator<Item = char>;
        fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char>;
        fn prefix_search<I>(&self, s:I) -> Vec<String> where I:Iterator<Item = char>;
    }
}

// pub use trie::{Trie,ArrayMapTrie};
// // pub use trie::{Trie,HashMapTrie,ArrayMapTrie,BTreeMapTrie};

// mod vec_sorted_map;
// pub use vec_sorted_map::VecSortedMap;
