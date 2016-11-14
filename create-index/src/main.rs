//! Authors: Kuan Yu, 3913893; Erik Schill, 3932609.
//! Honor Code:  We pledge that this program represents our own work.

extern crate conllx;
extern crate protocoll;

// use std::env::args_os;
use std::fs::File;
use std::io::{BufReader,BufWriter,Write};
use conllx::{Reader,Sentence};
use std::collections::HashMap;
use protocoll::{Map,Str};

fn main() {
    // let mut args = args_os();

    // match (args.nth(1), args.nth(2)) {
    //     (Some(conllx_in), Some(index_out)) => println!("yes"),

    //     _ => println!("usage: create-index CONLLX_PATH INDEX_PATH"),
    // }

    let conllx_in_path = "tubadw-r1-ir-sample-100000";
    let index_out_path = "index.txt";

    let conllx_in = File::open(conllx_in_path).unwrap();
    let index_out = File::create(index_out_path).unwrap();

    let lem2idxs:HashMap<String,Vec<u32>> =
        Reader::new(BufReader::new(conllx_in)).into_iter()
        .flat_map(|res_sent| match res_sent {
            Ok(sent) => sent,
            Err(err) => { println!("error: {}",err); Sentence::new(Vec::new())}})
        .filter_map(|tok| match (tok.lemma(),tok.features()) {
            (Some(lem),Some(feats)) => Some((lem.to_string(),feats.to_string())),
            _ => { println!("skipping: {}",tok); Option::None }})
        .filter_map(|(lem,feats)| match feats.as_str().parse::<u32>() {
            Ok(idx) => Option::Some((lem,idx)),
            Err(_) => { println!("illformed: {}",feats); Option::None }})
        .fold(HashMap::new(), |m,(k,i)| Map::update
              (m, k, |opt_v| SortedSet::inc
               (opt_v.unwrap_or_default(), i)));
    
    let mut wtr = BufWriter::new(index_out);
    for (lem,idxs) in &lem2idxs {
        let line = idxs.iter()
            .map(u32::to_string)
            .fold(lem.clone().inc('\t'), |line,s| line.plus(&s).inc(' '))
            .dec().inc('\n');
        if let Err(err) = wtr.write(line.as_bytes()) {
            println!("error: {}",err);
        }
    }
}

pub trait SortedSet<T> where T:Ord {
    fn inc(self, i:T) -> Self;
}

impl<T> SortedSet<T> for Vec<T> where T:Ord {
    fn inc(mut self, i:T) -> Self {
        if let Err(idx) = self.binary_search(&i) {
            Vec::insert(&mut self,idx,i);
        }
        self
    }
}
