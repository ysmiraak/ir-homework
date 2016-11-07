extern crate conllx;

// use std::env::args_os;
use std::fs::File;
use std::io::{BufReader,BufWriter};
use std::io::Write;
use conllx::Reader;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::BuildHasher;

trait SortedSet<T:Ord+Copy> {
    fn add(&mut self, item:T);
}

impl<T> SortedSet<T> for Vec<T> where T:Ord+Copy {
    fn add(&mut self, item:T) {
        match self.binary_search(&item) {
            Ok(_) => (),
            Err(idx) => self.insert(idx, item),
        };
    }
}

fn update<K,V,S,X>(mut m:HashMap<K,V,S>, k:K, f:&Fn(Option<V>,X) -> V, x:X) -> HashMap<K,V,S>
    where K:Hash+Eq, S:BuildHasher {
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

    let file_in = match File::open(conllx_in) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    let file_out = match File::create(index_out) {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };
    
    let sorted_set_add = |s,i| {
        let mut s = match s {
            Some(s) => s,
            None => Vec::new(),
        };
        s.add(i);
        s
    };
    
    let lem2idx = Reader::new(BufReader::new(file_in)).into_iter()
        .flat_map(|sent| match sent {
            Ok(sent) => sent,
            Err(err) => panic!("{}", err),
        })
        .map(|tok| match (tok.lemma(), tok.features()) {
            (Some(lem), Some(feat)) =>
                (String::from(lem), match feat.as_str().parse::<usize>() {
                    Ok(idx) => idx,
                    Err(err) => panic!("{}", err),
                }),
            _ => panic!("ill-formed entry!"),
        })
        .fold(HashMap::new(), |m,(k,x)| update(m,k,&sorted_set_add,x));

    let mut wtr = BufWriter::new(file_out);
    
    for (key, val) in lem2idx {
        write!(wtr, "{}\t", key).unwrap();
        for idx in val {
            write!(wtr, "{} ", idx).unwrap();
        }
        writeln!(wtr, "").unwrap();
    }
}
