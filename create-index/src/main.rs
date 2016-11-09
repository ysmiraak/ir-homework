extern crate conllx;

// use std::env::args_os;
use std::fs::File;
use std::io::{BufReader,BufWriter};
use std::io::Write;
use conllx::Reader;
use std::collections::HashMap;
use std::hash::Hash;

trait SortedSet<T:Ord+Copy> {
    fn inc(self, item:T) -> Self;
}

impl<T> SortedSet<T> for Vec<T> where T:Ord+Copy {
    fn inc(mut self, item:T) -> Self {
        match self.binary_search(&item) {
            Ok(_) => (),
            Err(idx) => self.insert(idx, item)
        };
        self
    }
}

fn update<K,V>(mut m:HashMap<K,V>, k:K, f:&Fn(Option<V>) -> V) -> HashMap<K,V>
    where K:Hash+Eq {
    let v = m.remove(&k);
    m.insert(k,f(v));
    m
}

fn main() {
    // let mut args = args_os();

    // match (args.nth(1), args.nth(2)) {
    //     (Some(conllx_in), Some(index_out)) => println!("yes"),

    //     _ => println!("usage: create-index CONLLX_PATH INDEX_PATH"),
    // }

    let conllx_in_file_name = "tubadw-r1-ir-sample-100000";
    let index_out_file_name = "index.txt";

    let conllx_in = File::open(conllx_in_file_name).unwrap();
    let index_out = File::create(index_out_file_name).unwrap();

    let ref grow_posting = |mv:Option<Vec<usize>>, i:usize| match mv {
        Some(v) => v.inc(i), None => Vec::new().inc(i) };

    let lem2idx = Reader::new(BufReader::new(conllx_in)).into_iter()
        .flat_map(|sent| sent.unwrap())
        .filter_map(|tok| {
            match (tok.lemma(), tok.features()) {
                (Some(lem), Some(feat)) =>
                    match feat.as_str().parse::<usize>() {
                        Ok(idx) => Option::Some((String::from(lem),idx)),
                        Err(_) => { println!("{:?}",tok); Option::None }
                    },
                _ => { println!("{:?}",tok); Option::None}
            }
        })
        .fold(HashMap::new(), |m,(k,i)| update(m, k, &|mv| grow_posting(mv,i)));
    
    let mut wtr = BufWriter::new(index_out);

    for (key, val) in lem2idx {
        write!(wtr, "{}\t", key).unwrap();
        for idx in val {
            write!(wtr, "{} ", idx).unwrap();
        }
        writeln!(wtr, "").unwrap();
    }
}
