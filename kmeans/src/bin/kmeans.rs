// Author: Kuan Yu, 3913893
// Honor Code:  I pledge that this program represents my own work.

extern crate kmeans;
extern crate getopts;
extern crate rust2vec;

use getopts::Options;
use std::fs::File;
use std::path::Path;
use std::env::args;
use std::process::exit;
use std::io::{BufReader, BufWriter, Write};
use rust2vec::{Embeddings, ReadWord2Vec};
use kmeans::{kmeans, step_sample, arg_max};

fn main() {
    let (path_in, path_out, opt_k, epsilon, max_iter, verbose) = {
        let mut opts = Options::new();
        opts.reqopt("i", "input", "the binary embeddings file.", "")
            .optopt("o", "output", "the output tsv file; default: `word_cluster.tsv`.", "")
            .optopt("k", "centers", "the number of clusters; default: `sqrt(|data|)`.", "")
            .optopt("e", "epsilon", "tolerance for convergence; default: `0.05`.", "")
            .optopt("m", "max-iter", "the maximum number of iterations; default: `25`.", "")
            .optopt("v", "verbose", "`false` or `true` by default.", "");

        let matches = match opts.parse(args().skip(1)) {
            Err(e) => {
                println!("{}", opts.usage(&e.to_string()));
                exit(1)
            }
            Ok(m) => m,
        };

        (matches.opt_str("i").unwrap(),
         matches.opt_str("o").unwrap_or("word_cluster.tsv".to_owned()),
         matches.opt_str("k").unwrap_or_default().parse::<usize>().ok(),
         matches.opt_str("e").unwrap_or_default().parse::<f32>().unwrap_or(0.05),
         matches.opt_str("m").unwrap_or_default().parse::<usize>().unwrap_or(25),
         matches.opt_str("v").unwrap_or_default().parse::<bool>().unwrap_or(true))
    };

    if verbose { println!("loading embeddings ...");}
    let embeddings = {
        let mut emb = Embeddings::read_word2vec_binary(&mut BufReader::new(open_file(&path_in)))
            .unwrap();
        emb.normalize();
        emb
    };

    let centroids = {
        let data = embeddings.data();
        let k = opt_k.unwrap_or(f32::sqrt(data.rows() as f32) as usize);
        if verbose { println!("number of clusters:\t{}", k);}
        kmeans(&data, step_sample(&data, k), epsilon, max_iter, verbose)
    };

    let mut wtr = BufWriter::new(create_file(&path_out));
    if verbose { println!("writing to {} ...", path_out);}
    for (word, embedding) in embeddings.iter() {
        writeln!(wtr, "{}\t{}", word, arg_max(&centroids.dot(&embedding))).unwrap();
    }
}

pub fn open_file<P>(path: P) -> File
    where P: AsRef<Path>
{
    match File::open(&path) {
        Err(_) => {
            println!("cannot open file: {:?}", path.as_ref());
            exit(1)
        }
        Ok(file) => file,
    }
}

pub fn create_file<P>(path: P) -> File
    where P: AsRef<Path>
{
    match File::create(&path) {
        Err(_) => {
            println!("cannot create file: {:?}", path.as_ref());
            exit(1)
        }
        Ok(file) => file,
    }
}
