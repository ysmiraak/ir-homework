//! Author: Kuan Yu, 3913893
//! Honor Code: I pledge that this program represents my own work.

extern crate conllx;
extern crate protocoll;

use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufWriter,Write};
use conllx::{Reader,Sentence};
use std::collections::HashMap;
use protocoll::{Map,Str};

fn main() {
    // let args:Vec<&str> = vec!["create-index","tubadw-r1-ir-sample-100000","index.txt"];

    let args:Vec<String> = args().collect();
    if 3 != args.len() {
        println!("usage: {} INPUT_CONLLX_FILE OUTPUT_INDEX_FILE", args[0]); exit(1)
    }
    
    let conllx_in = match File::open(&args[1]) {
        Err(_) => { println!("cannot open file for reading: {}", args[1]); exit(2)}
        Ok(file) => file
    };
    
    let index_out = match File::create(&args[2]) {
        Err(_) => { println!("cannot open file for reading: {}", args[2]); exit(3)}
        Ok(file) => file
    };

    let term2idxs:HashMap<String,Vec<u32>> =
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
    for (term,idxs) in &term2idxs {
        let line = idxs.iter()
            .map(u32::to_string)
            .fold(term.clone().inc('\t'), |line,s| line.plus(&s).inc(' '))
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
