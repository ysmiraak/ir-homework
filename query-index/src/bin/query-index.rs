extern crate query_index;
extern crate porter_stemmer;
extern crate stemmer;
extern crate getopts;

use query_index::query_processor::{QueryProcessor, identity_tf, binary_tf, sublinear_tf};
use query_index::inverted_index::InvertedIndex;
use query_index::sparse_dense_vec::DenseVec;
use query_index::error::{LoadError, FormatError};
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

    do_query(BufReader::new(open_file(&matches.opt_str("i").unwrap())),
             BufReader::new(open_file(&matches.opt_str("t").unwrap())),
             choose_stemmer(&matches.opt_str("s").unwrap_or("none".to_owned())),
             choose_weighting(&matches.opt_str("w").unwrap_or("identity".to_owned())))
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

fn choose_stemmer(alt: &str) -> Box<FnMut(&str) -> String> {
    match alt.as_ref() {
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
    }
}

fn choose_weighting(alt: &str) -> fn(usize) -> f64 {
    match alt.as_ref() {
        "identity" => identity_tf,
        "binary" => binary_tf,
        "sublinear" => sublinear_tf,
        unk => {
            println!("unknown weighting: {}", unk);
            exit(1)
        }
    }
}

fn do_query<R>(index: R, titles: R,
               mut stem: Box<FnMut(&str) -> String>,
               weight_tf: fn(usize) -> f64)
    where R:BufRead
{
    let doc2titles = load_titles(titles).unwrap();
    let inv_index = InvertedIndex::load(index).unwrap();
    let processor = QueryProcessor::new(&inv_index, doc2titles.len(), weight_tf);

    let stdin = stdin();
    for res_line in stdin.lock().lines() {
        let query = res_line.unwrap().split_whitespace()
            .map(|s| stem(&s.to_lowercase()))
            .collect::<Vec<_>>();
        for term in &query {
            println!("idf({}) = {}", term, processor.idf(term))
        }
        let mut results = processor.process(&query);
        let missing_title = String::new();
        let mut n = 0;
        while n < MAX_MATCH {
            n += 1;
            match results.pop() {
                Some(doc_sim) =>
                    println!("{} ({}): {}",
                             doc_sim.doc(), doc_sim.sim(),
                             doc2titles.get(doc_sim.doc()).unwrap_or(&missing_title)),
                None => break
            }
        }
    }
}

fn load_titles<R>(rdr: R) -> Result<DenseVec<String>, LoadError> where R: BufRead {
    let mut ret = DenseVec::new();
    for res_line in rdr.lines() {
        let line = try!(res_line);
        let x = try!(line.find('\t').ok_or(FormatError::new(&line)));
        ret.insert(try!(line[..x].parse::<usize>()), line[x+1..].to_string());
    }
    Ok(ret.shrink())
}
