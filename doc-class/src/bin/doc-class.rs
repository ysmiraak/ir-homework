extern crate doc_class;
extern crate getopts;
extern crate conllx;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use conllx::{Reader, Sentence};
use doc_class::numberer::{Numberer, HashMapNumberer};
use doc_class::inverted_index::{InvertedIndex, binary, tf_idf, btf_idf, stf_idf};
use doc_class::io_utils::{open_file, create_file, iter_file_paths};
use getopts::{Options};
use std::env::args;
use std::process::exit;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let (path_in, path_out, n1, n2, n3, min_freq, feat_fn) = {
        let mut opts = Options::new();
        opts.reqopt("i", "input", "with grouped conll files.", "DIRECTORY")
            .optopt("o", "output", "for the output, or `data.svm` by default.", "FILENAME")
            .optopt("1", "unigram", "for hashing unigrams, or `2^18` by default.", "DIMENSIONS")
            .optopt("2", "bigram", "for hashing bigrams, or `0` by default.", "DIMENSIONS")
            .optopt("3", "trigram", "for hashing trigrams, or `0` by default.", "DIMENSIONS")
            .optopt("m", "min-freq", "for ngram frequencies, or `1` by default.", "THRESHOLD")
            .optopt("f", "feature", "`binary`, `tfidf`, `btfidf`, or `stfidf` by default.",
                    "FEATURE");
        let matches = match opts.parse(args().skip(1)) {
            Err(e) => {
                println!("{}", opts.usage(&e.to_string()));
                exit(1)
            }
            Ok(m) => m,
        };
        (matches.opt_str("i").unwrap(),
         matches.opt_str("o").unwrap_or("data.svm".to_owned()),
         matches.opt_str("1").unwrap_or_default().parse::<usize>().unwrap_or(0),
         matches.opt_str("2").unwrap_or_default().parse::<usize>().unwrap_or(0),
         matches.opt_str("3").unwrap_or_default().parse::<usize>().unwrap_or(0),
         matches.opt_str("m").unwrap_or_default().parse::<usize>().unwrap_or(1),
         match matches.opt_str("f").unwrap_or("stfidf".to_owned()).as_ref() {
             "binary" => binary,
             "tfidf" => tf_idf,
             "btfidf" => btf_idf,
             "stfidf" => stf_idf,
             unk => {
                 println!("unknown feature: {}", unk);
                 exit(1)
             }
         })
    };
    
    println!("unigram dim: {}", n1);
    println!("bigram  dim: {}", n2);
    println!("trigram dim: {}", n3);

    let (labels, features) = {
        let file_paths = iter_file_paths(path_in);
        let mut classes = HashMapNumberer::new();
        let mut labels = Vec::new();
        let mut inv_idx = InvertedIndex::new();
        inv_idx.ensure_size(n1 + n2 + n3);
        
        for file_path in file_paths {
            match file_path.parent()
                .and_then(Path::file_name)
                .and_then(OsStr::to_str) {
                    None => continue,
                    Some(label) => labels.push(classes.number(label))
                }

            let tokens = Reader::new(BufReader::new(open_file(&file_path))).sentences()
                .flat_map(|res_sent| res_sent.unwrap_or(Sentence::new(Vec::new())))
                .map(|tok| tok.form().unwrap_or("").to_owned())
                .collect::<Vec<_>>();

            let sentinel = Vec::<usize>::new();
            let mut terms: Box<Iterator<Item = usize>> =
                Box::new(sentinel.iter().map(ToOwned::to_owned));
            if n1 > 0 {
                terms = Box::new(terms.chain(tokens.iter()
                                             .map(|x| hash_code(x) % n1)));
            }
            if n2 > 0 {
                terms = Box::new(terms.chain(tokens.iter()
                                             .zip(tokens.iter().skip(1))
                                             .map(|x| n1 + (hash_code(x) % n2))));
            }
            if n3 > 0 {
                terms = Box::new(terms.chain(tokens.iter()
                                             .zip(tokens.iter().skip(1))
                                             .zip(tokens.iter().skip(2))
                                             .map(|x| n1 + n2 + (hash_code(x) % n3))));
            }
            
            inv_idx.inv_push(terms);
        }

        (labels, inv_idx.doc_features(feat_fn, min_freq))
    };

    let mut wtr = BufWriter::new(create_file(path_out));
    for (label, feature) in labels.iter().zip(features.iter()) {
        write!(wtr, "{}", label).unwrap();
        for &(idx, feat) in feature {
            write!(wtr, " {}:{}", idx + 1, feat).unwrap();
        }
        writeln!(wtr).unwrap();
    }
}

fn hash_code<T>(x: T) -> usize where T: Hash {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish() as usize
}
