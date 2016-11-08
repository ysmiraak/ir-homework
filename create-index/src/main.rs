extern crate conllx;

// use std::env::args_os;
use std::fs::File;
use std::io::{BufReader,BufWriter};
use std::io::Write;
use conllx::Reader;
use std::collections::HashMap;
use std::hash::Hash;

trait SortedSet<T:Ord+Copy> {
    fn grow(self, item:T) -> Self;
}

impl<T> SortedSet<T> for Vec<T> where T:Ord+Copy {
    fn grow(mut self, item:T) -> Self {
        match self.binary_search(&item) {
            Ok(_) => (),
            Err(idx) => self.insert(idx, item)
        };
        self
    }
}

fn update<K,V,X>(mut m:HashMap<K,V>, k:K, f:&Fn(Option<V>,X) -> V, x:X) -> HashMap<K,V>
    where K:Hash+Eq {
    let v = m.remove(&k);
    m.insert(k,f(v,x));
    m
}

fn main() {
    // let mut args = args_os();

    // match (args.nth(1), args.nth(2)) {
    //     (Some(conllx_in), Some(index_out)) => println!("yes"),
    
    //     _ => println!("usage: create-index CONLLX_PATH INDEX_PATH"),
    // }

    let conllx_in = "tubadw-r1-ir-sample-1000";
    let index_out = "index.txt";

    let file_in = File::open(conllx_in).unwrap();

    let file_out = File::create(index_out).unwrap();

    let ref grow_posting = |s:Option<Vec<usize>>, i:usize| match s {
        Some(s) => s.grow(i), None => Vec::new() };
        
    let lem2idx = Reader::new(BufReader::new(file_in)).into_iter()
        .flat_map(|sent| sent.unwrap())
        .map(|tok| (String::from(tok.lemma().unwrap()),
                    tok.features().unwrap().as_str().parse::<usize>().unwrap()))
        .fold(HashMap::new(), |m,(k,x)| update(m, k, grow_posting, x));

    let mut wtr = BufWriter::new(file_out);
    
    for (key, val) in lem2idx {
        write!(wtr, "{}\t", key).unwrap();
        for idx in val {
            write!(wtr, "{} ", idx).unwrap();
        }
        writeln!(wtr, "").unwrap();
    }
}
