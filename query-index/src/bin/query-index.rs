extern crate query_index;
extern crate porter_stemmer;
extern crate stemmer;
extern crate getopts;

use query_index::query_processor::{QueryProcessor, identity_tf, binary_tf, sublinear_tf};
use query_index::inverted_index::InvertedIndex;
use query_index::sparse_dense_vec::DenseVec;
use porter_stemmer::stem;
use stemmer::Stemmer;
use getopts::Options;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader, BufRead, stdin};

const MAX_MATCH: usize = 5;

fn main() {
    let mut opts = Options::new();
    opts.reqopt("i", "index", "of the inverted index.", "INDEX_FILE")
        .reqopt("t", "titles", "of the document titles.", "TITLES_FILE")
        .optopt("s", "stemmer",
                "used for the index: `snowball`, `porter`, or `none` by default.",
                "STEMMER")
        .optopt("w", "weighting",
                "for the term frequencies: `binary`, `sublinear`, or `identity` by default.",
                "WEIGHTING");

    let matches = match opts.parse(args().skip(1)) {
        Err(e) => {
            println!("{}", opts.usage(&e.to_string()));
            exit(1)
        }
        Ok(m) => m,
    };

    let i = BufReader::new(open_file(&matches.opt_str("i").unwrap()));
    let t = BufReader::new(open_file(&matches.opt_str("t").unwrap()));
    let s: Box<FnMut(&str) -> String> =
        match matches.opt_str("s").unwrap_or("none".to_owned()).as_ref() {
            "none" => Box::new(|s| s.to_owned()),
            "porter" => Box::new(|s| stem(s)),
            "snowball" => {
                let mut stemmer = Stemmer::new("english").unwrap();
                Box::new(move |s| stemmer.stem(s))
            }
            unk => {
                println!("unknown stemmer: {}", unk);
                exit(1)
            }
        };
    let w = match matches.opt_str("w").unwrap_or("identity".to_owned()).as_ref() {
        "identity" => identity_tf,
        "binary" => binary_tf,
        "sublinear" => sublinear_tf,
        unk => {
            println!("unknown weighting: {}", unk);
            exit(1)
        }
    };
    do_query(i, t, s, w)
}

fn open_file(path: &str) -> File {
    match File::open(path) {
        Err(_) => {
            println!("cannot open file: {}", path);
            exit(1)
        }
        Ok(file) => file,
    }
}

fn do_query<R,W>(index: R, titles: R, mut stem: Box<FnMut(&str) -> String>, weight: W)
    where R:BufRead, W: Fn(usize) -> f64
{
    let doc2titles = load_titles(titles);
    let inv_index = InvertedIndex::load(index);
    let processor = QueryProcessor::new(&inv_index, doc2titles.len(), &weight);

    let stdin = stdin();
    for res_line in stdin.lock().lines() {
        let query = res_line.unwrap().split_whitespace()
            .map(|s| stem(&s.to_lowercase()))
            .collect::<Vec<_>>();
        for term in &query {
            println!("idf({}) = {}", term, processor.idf(term))
        }
        let mut results = processor.process(&query);
        let mut n = 0;
        while n < MAX_MATCH {
            n += 1;
            match results.pop() {
                Some(doc_sim) =>
                    println!("{} ({}): {}",
                             doc_sim.doc(), doc_sim.sim(),
                             doc2titles.get(doc_sim.doc()).unwrap()),
                None => break
            }
        }
    }
}

fn load_titles<R>(rdr: R) -> DenseVec<String> where R: BufRead {
    rdr.lines()
        .map(|res_line| {
            let line = res_line.unwrap();
            let x = line.find('\t').unwrap();
            (line[..x].parse::<usize>().unwrap(), line[x + 1..].to_string())
        }).collect()
}
