use sparse_dense_vec::SparseVec;
use error::{LoadError, FormatError};
use std::collections::HashMap;
use std::io::{BufRead};
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

    pub fn load<R>(rdr: R) -> Result<InvertedIndex, LoadError>
        where R: BufRead
    {
        let mut content = HashMap::new();
        for res_line in rdr.lines() {
            let line = try!(res_line);
            let x = try!(line.find('\t').ok_or(FormatError::new(&line)));
            let mut doc2tf = PostingList::new();
            for (doc, tf) in line[x+1..].split_whitespace().tuples() {
                doc2tf.insert(try!(str::parse(doc)), try!(str::parse(tf)));
            }
            doc2tf.shrink_to_fit();
            content.insert(line[..x].to_string(), doc2tf);
        }
        content.shrink_to_fit();
        Ok(InvertedIndex(content))
    }

    pub fn view_content(&self) -> &HashMap<String, PostingList> {
        &self.0
    }
}
