extern crate conllx;
extern crate protocoll;

use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufWriter,Write};
use conllx::{Reader,Sentence};
use std::collections::HashMap;
use protocoll::{MapMut,Str};
use protocoll::set::VecSortedSet;

fn main() {
    // let args:Vec<&str> = vec!["create-index","tubadw-r1-ir-sample-100000","index.txt"];

    let args:Vec<String> = args().collect();
    if 3 != args.len() {
        println!("usage: {} INPUT_CONLLX_FILE OUTPUT_INDEX_FILE",args[0]); exit(1)
    }

    let conllx_in = match File::open(&args[1]) {
        Err(_) => {println!("cannot open file for reading: {}",args[1]); exit(2)}
        Ok(file) => file
    };

    let index_out = match File::create(&args[2]) {
        Err(_) => {println!("cannot open file for reading: {}",args[2]); exit(3)}
        Ok(file) => file
    };

    let term2idxs:HashMap<String,VecSortedSet<u32>> =
        Reader::new(BufReader::new(conllx_in)).into_iter()
        .flat_map(|res_sent| match res_sent {
            Ok(sent) => sent,
            Err(err) => { println!("error: {}",err); Sentence::new(Vec::new())}})
        .filter_map(|tok| match (tok.lemma(),tok.features()) {
            (Some(lem),Some(feats)) => Some((lem.to_string(),feats.to_string())),
            _ => { println!("skipping: {}",tok); Option::None}})
        .filter_map(|(lem,feats)| match feats.as_str().parse::<u32>() {
            Ok(idx) => Option::Some((lem,idx)),
            Err(_) => { println!("illformed: {}",feats); Option::None}})
        .fold(HashMap::new(), |mut m, (k, i)|
              {m.update_mut(k, VecSortedSet::new(), |s| {s.insert(i);}); m});

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
