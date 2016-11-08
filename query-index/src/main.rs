use std::cmp::min;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};
use std::collections::HashMap;
use std::hash::Hash;

trait Posting<T:Posting<T>> {
    fn intersect(&self, other: &T) -> Self;
}

impl<T> Posting<Vec<T>> for Vec<T> where T:Ord+Copy {
    fn intersect(&self, other: &Vec<T>) -> Self {
        let mut inter = Vec::with_capacity(min(self.len(), other.len()));

        let (smaller, larger) = if self.len() < other.len() {
            (self, other)
        } else {
            (other, self)
        };

        let mut offset = 0;
        for doc in smaller.into_iter() {
            offset = match larger[offset..].binary_search(doc) {
                Ok(idx) => {
                    inter.push(*doc);
                    idx
                }
                Err(idx) => idx,
            }
        }
        inter
    }
}

fn parse_into_map<K,V,R>(rdr:R, f:&Fn(&String) -> (K,V)) -> HashMap<K,V>
    where K:Hash+Eq, R:BufRead {
    rdr.lines()
        .filter_map(|line_res| match line_res {
            Ok(line) => Option::Some(f(&line)),
            Err(_) => Option::None })
        .fold(HashMap::new(), |mut m,(k,v)| { m.insert(k,v); m })
}

fn parse_posting(line:&String) -> (String, Vec<usize>) {
    let x = line.find('\t').unwrap();
    let term = line[..x].to_string();
    let list = line[x+1..].split_whitespace()
        .map(|idx_str| idx_str.parse::<usize>())
        .filter_map(|idx_res| match idx_res {
            Ok(idx) => Option::Some(idx),
            Err(_) => Option::None })
        .collect();
    (term, list)
}

fn parse_idx2doc(line:&String) -> (usize, String) {
    let x = line.find('\t').unwrap();
    let idx = line[..x].parse::<usize>().unwrap();
    let doc = line[x+1..].to_string();
    (idx, doc)
}

fn main() {
    let posting_file_name = "index.txt";
    let idx2doc_file_name = "tubadw-r1-ir-ids-1000.tab";

    let posting_in = File::open(posting_file_name).unwrap();
    let idx2doc_in = File::open(idx2doc_file_name).unwrap();
    // try to open both files before parsing either
    let lem2idx = parse_into_map(BufReader::new(posting_in), &parse_posting);
    let idx2doc = parse_into_map(BufReader::new(idx2doc_in), &parse_idx2doc);

    let stdin = stdin();
    println!("enter query:");
    for line in stdin.lock().lines() {        
        let mut terms = Vec::new();
        for term in line.unwrap().split_whitespace() {
            if term.is_empty() { continue }
            match lem2idx.get(term) {
                Some(posting) => terms.push(posting),
                None => { println!("no match found.\n\nenter query:"); break }
            }
        }
        if terms.len() < 1 { continue }
        // sort and do AND query
        terms.sort_by(|a,b| a.len().cmp(&b.len()));
        let posting = terms[1..].iter()
            .fold(Cow::Borrowed(terms[0]), |a,b| Cow::Owned((&a).intersect(b)))
            .into_owned();
        for idx in posting {
            println!("{}: {}", idx, idx2doc.get(&idx).unwrap());
        }
        println!("\n\nenter query:");
    }
}
