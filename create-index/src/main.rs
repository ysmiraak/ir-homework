extern crate conllx;

// use std::env::args_os;
use std::fs::File;
use conllx::{Reader,Sentence};
use std::io::{BufReader,BufWriter};
use std::io::Write;
use std::collections::HashMap;

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
        .fold(HashMap::new(), |mut m,(k,i)| {
            let mut v = m.remove(&k).unwrap_or(Vec::new());
            SortedSet::insert(&mut v,i);
            m.insert(k,v);
            m
        });
    
    let mut wtr = BufWriter::new(index_out);
    for (lem,idxs) in lem2idxs {
        let mut out = lem;
        out.push(' ');
        out.push_str(&idxs.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

        if let Err(err) = writeln!(wtr, "{}", out) {
            println!("error: {}",err);
        }
    }
}

pub trait SortedSet<T> where T:Ord {
    fn insert(&mut self, i:T);
}

impl<T> SortedSet<T> for Vec<T> where T:Ord {
    fn insert(&mut self, i:T) {
        if let Err(idx) = self.binary_search(&i) {
            Vec::insert(self,idx,i);
        }
    }
}
