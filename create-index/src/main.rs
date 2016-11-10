extern crate protocoll;
extern crate conllx;

use protocoll::Map;
// use std::env::args_os;
use std::fs::File;
use conllx::Reader;
use std::io::{BufReader,BufWriter};
use std::io::Write;
use std::collections::HashMap;

fn main() {
    // let mut args = args_os();

    // match (args.nth(1), args.nth(2)) {
    //     (Some(conllx_in), Some(index_out)) => println!("yes"),

    //     _ => println!("usage: create-index CONLLX_PATH INDEX_PATH"),
    // }

    let conllx_in_path = "tubadw-r1-ir-sample-1000";
    let index_out_path = "index.txt";

    let conllx_in = File::open(conllx_in_path).unwrap();
    let index_out = File::create(index_out_path).unwrap();

    let lem2ids:HashMap<String,Vec<u32>> =
        Reader::new(BufReader::new(conllx_in)).into_iter()
        .flat_map(|sent| sent.unwrap())
        .filter_map(|tok| match (tok.lemma(), tok.features()) {
            (Some(lem), Some(feat)) =>
                match feat.as_str().parse::<u32>() {
                    Ok(idx) => Option::Some((String::from(lem),idx)),
                    Err(_) => { println!("illformed: {}",tok); Option::None }},
            _ => { println!("skipping: {}",tok); Option::None }}
        )
        .fold(HashMap::new(), |m,(k,i)| Map::update
              (m, k, |opt_v| SortedSet::inc
               (if let Some(v) = opt_v {v} else {Vec::new()}, i)));

    let mut wtr = BufWriter::new(index_out);

    for (key,val) in lem2ids {
        write!(wtr, "{}\t", key).unwrap();
        for idx in val {
            write!(wtr, "{} ", idx).unwrap();
        }
        writeln!(wtr, "").unwrap();
    }
}

pub trait SortedSet<T> where T:Ord+Copy {
    fn inc(self, i:T) -> Self;
}

impl<T> SortedSet<T> for Vec<T> where T:Ord+Copy {
    fn inc(mut self, i:T) -> Self {
        if let Err(idx) = self.binary_search(&i) {
            self.insert(idx,i);
        }
        self
    }
}
