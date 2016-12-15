use protocoll::Map;
use sparse_dense_vec::SparseVec;
use std::collections::HashMap;
use std::io::BufRead;
use itertools::Itertools;

pub type PostingList = SparseVec<usize>;

#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct InvertedIndex(HashMap<String, PostingList>);

impl InvertedIndex {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get<'a>(&'a self, term: &str) -> Option<&'a PostingList> {
        self.0.get(term)
    }

    pub fn load<R>(rdr: R) -> InvertedIndex
        where R: BufRead
    {
        InvertedIndex
            (rdr.lines()
             .map(|res_line| {
                 let line = res_line.unwrap();
                 let x = line.find('\t').unwrap();
                 (line[..x].to_string(),
                  line[x + 1..].split_whitespace()
                  .map(str::parse)
                  .map(Result::unwrap)
                  .tuples().collect::<PostingList>().shrink())
             }).collect::<HashMap<_,_>>().shrink())
    }

    pub fn view_content(&self) -> &HashMap<String, PostingList> {
        &self.0
    }
}
