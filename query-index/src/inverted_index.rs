// use protocoll::Map;
use sparse_dense_vec::SparseVec;
use std::collections::HashMap;
use std::io::{self,BufRead};
use std::num;
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

    pub fn load<R>(rdr: R) -> Result<InvertedIndex,LoadError>
        where R: BufRead
    {
        let mut content = HashMap::new();
        for res_line in rdr.lines() {
            let line = try!(res_line);
            let x = try!(line.find('\t').ok_or(LoadError::Format(line.to_owned())));
            
            let mut doc2tf = PostingList::new();
            for (doc, tf) in line[x + 1..].split_whitespace().tuples() {
                doc2tf.insert(try!(str::parse(doc)), try!(str::parse(tf)));
            }
            doc2tf.shrink_to_fit();

            // line[x + 1..].split_whitespace().map(str::parse);
            
            content.insert(line[..x].to_string(), doc2tf);
        }
        content.shrink_to_fit();
        Ok(InvertedIndex(content))
        // Ok(InvertedIndex
        //    (rdr.lines()
        //     .map(|res_line| {
        //         let line = try!(res_line);
        //         let x = line.find('\t').unwrap();
        //         (line[..x].to_string(),
        //          line[x + 1..].split_whitespace()
        //          .map(str::parse)
        //          .map(Result::unwrap)
        //          .tuples().collect::<PostingList>().shrink())
        //     }).collect::<HashMap<_,_>>().shrink()))
    }

    // pub fn load<R>(rdr: R) -> InvertedIndex
    //     where R: BufRead
    // {
    //     InvertedIndex
    //         (rdr.lines()
    //          .map(|res_line| {
    //              let line = res_line.unwrap();
    //              let x = line.find('\t').unwrap();
    //              (line[..x].to_string(),
    //               line[x + 1..].split_whitespace()
    //               .map(str::parse)
    //               .map(Result::unwrap)
    //               .tuples().collect::<PostingList>().shrink())
    //          }).collect::<HashMap<_,_>>().shrink())
    // }

    pub fn view_content(&self) -> &HashMap<String, PostingList> {
        &self.0
    }
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Format(String)
}

// use std::error::Error;
// use std::fmt;

// impl fmt::Display for LoadError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             LoadError::Io(ref err) => write!(f, "IO error: {}", err),
//             LoadError::Parse(ref err) => write!(f, "Parse error: {}", err),
//             LoadError::Format(ref s) => write!(f, "Illformed: {}", s)
//         }
//     }
// }

// impl Error for LoadError {
//     fn description(&self) -> &str {
//         match *self {
//             LoadError::Io(ref err) => err.description(),
//             LoadError::Parse(ref err) => err.description(),
//             LoadError::Format(ref s) => s
//         }
//     }

//     fn cause(&self) -> Option<&Error> {
//         match *self {
//             LoadError::Io(ref err) => Some(err),
//             LoadError::Parse(ref err) => Some(err),
//             LoadError::Format(ref s) => Some(s)
//         }
//     }
// }

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> LoadError {
        LoadError::Io(err)
    }
}

impl From<num::ParseIntError> for LoadError {
    fn from(err: num::ParseIntError) -> LoadError {
        LoadError::Parse(err)
    }
}
