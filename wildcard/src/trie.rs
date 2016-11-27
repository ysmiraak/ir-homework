pub trait Trie {
    fn learn<I>(self, s:I) -> Self where I:Iterator<Item = char>;
    fn recognize<I>(&self, s:I) -> bool where I:Iterator<Item = char>;
    fn prefix_search<I>(&self, s:I) -> Vec<String> where I:Iterator<Item = char>;
}
