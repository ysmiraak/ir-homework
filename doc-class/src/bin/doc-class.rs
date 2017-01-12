extern crate doc_class;
extern crate getopts;
extern crate protocoll;
extern crate conllx;

use protocoll::Seq;
use conllx::{Reader, Sentence};
use doc_class::inverted_index::InvertedIndex;
use doc_class::numberer::{Numberer, HashMapNumberer};
// use doc_class::error::{LoadError, FormatError};
use getopts::Options;
use std::env::args;
use std::process::exit;
use std::fs::{read_dir, ReadDir, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() {
    let mut opts = Options::new();
    opts.reqopt("i", "input", "with grouped conll files.", "DIRECTORY")
        .optopt("o", "output", "for the output, or `data.svm` by default.", "FILENAME");

    let matches = match opts.parse(args().skip(1)) {
        Err(e) => {
            println!("{}", opts.usage(&e.to_string()));
            exit(1)
        }
        Ok(m) => m,
    };

    let mut wtr = BufWriter::new(create_file(&matches.opt_str("o").unwrap_or("data.svm".to_owned())));

    let file_paths = match iter_file_paths(&matches.opt_str("i").unwrap()) {
        Err(err) => {
            println!("cannot open directory: {}", err);
            exit(1)
        }
        Ok(file_paths) => file_paths
    };

    let (labels, features) = {
        let mut classes = HashMapNumberer::new();
        let mut labels = Vec::new();
        let mut inv_idx = InvertedIndex::new();

        for file_path in file_paths {
            // println!("opening: {:?}", file_path);
            // if !file_path.ends_with(".conll") {continue}
            // println!("reading: {:?}", file_path);
            let label = match file_path.parent() {
                None => continue,
                Some(folder) => match folder.file_name() {
                    None => continue,
                    Some(name) => name.to_str().unwrap()
                }
            };
            labels.push(classes.number(label));
            
            inv_idx = inv_idx.add_doc(
                Reader::new(BufReader::new(open_file(&file_path)))
                    .into_iter()
                    .flat_map(|res_sent| res_sent.unwrap_or(Sentence::new(Vec::new())))
                    .map(|tok| tok.form().unwrap_or("").to_owned())
            );
        }
        println!("doc count: {}", labels.len());
        println!("type count: {}", inv_idx.view_content().len());
        (labels, inv_idx.features())
    };

    for (label, idx2feat) in labels.iter().zip(features.iter()) {
        write!(wtr, "{}", label).unwrap();
        for &(idx, feat) in idx2feat {
            write!(wtr, " {}:{}", idx + 1, feat).unwrap();
        }
        writeln!(wtr).unwrap();
    }
}

// fn load_titles<R>(rdr: R) -> Result<DenseVec<String>, LoadError> where R: BufRead {
//     let mut ret = DenseVec::new();
//     for res_line in rdr.lines() {
//         let line = try!(res_line);
//         let x = try!(line.find('\t').ok_or(FormatError::new(&line)));
//         ret.insert(try!(line[..x].parse::<usize>()), line[x+1..].to_string());
//     }
//     Ok(ret.shrink())
// }

fn open_file<P>(path: P) -> File where P: AsRef<Path> {
    match File::open(&path) {
        Err(_) => {
            println!("cannot open file: {:?}", path.as_ref());
            exit(1)
        }
        Ok(file) => file,
    }
}

fn create_file<P>(path: P) -> File where P: AsRef<Path> {
    match File::create(&path) {
        Err(_) => {
            println!("cannot open file: {:?}", path.as_ref());
            exit(1)
        }
        Ok(file) => file,
    }
}

fn iter_file_paths<P>(path: P) -> io::Result<FilePaths> where P: AsRef<Path> {
    Ok(FilePaths(Vec::new().inc(try!(read_dir(path)))))
}

pub struct FilePaths(Vec<ReadDir>);

impl Iterator for FilePaths {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        match self.0.pop() {
            None => None,
            Some(mut dir) => {
                match dir.next() {
                    None => self.next(),
                    Some(res_entry) => {
                        self.0.push(dir);
                        match res_entry {
                            Err(_) => self.next(),
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_file() {
                                    Some(path)
                                } else {
                                    match read_dir(path) {
                                        Err(_) => self.next(),
                                        Ok(dir) => {
                                            self.0.push(dir);
                                            self.next()}}}}}}}}}
    }
}
