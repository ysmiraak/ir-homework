use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let posting_path = "index.txt";
    let idx2doc_path = "tubadw-r1-ir-ids-100000.tab";

    let posting_in = File::open(posting_path).unwrap();
    let idx2doc_in = File::open(idx2doc_path).unwrap();
    
    // try to open both files before parsing either
    let posting = parse_to_map(BufReader::new(posting_in), parse_posting);
    let idx2doc = parse_to_map(BufReader::new(idx2doc_in), parse_idx2doc);

    let stdin = stdin();
    println!("enter query:");
    for line_res in stdin.lock().lines() {
        if let Err(_) = line_res { continue }
        let mut terms = Vec::new();
        for term in line_res.unwrap().split_whitespace() {
            if term.is_empty() { continue }
            match posting.get(term) {
                Some(p) => terms.push(p),
                None => { println!("no match found.\n\nenter query:");
                          terms.clear();
                          break }
            }
        }
        if terms.is_empty() { continue }
        // sort and do AND query
        terms.sort_by(|a,b| a.len().cmp(&b.len()));
        let posting_list = terms[1..].iter()
            .fold(Cow::Borrowed(terms[0]), |a,b| Cow::Owned((&a).intersection(b)))
            .into_owned();
        for idx in posting_list {
            println!("{}: {}", idx, idx2doc[&idx]);
        }
        println!("\n\nenter query:");
    }
}

fn parse_to_map<F,K,V,R>(rdr:R, f:F) -> HashMap<K,V>
    where F:Fn(String) -> (K,V),
          K:Hash+Eq,
          R:BufRead,{
    rdr.lines()
        .filter_map(|res_line| match res_line {
            Ok(line) => Option::Some(f(line)),
            Err(_)   => Option::None })
        .fold(HashMap::new(), |mut m,(k,v)| { m.insert(k,v); m })
}

fn parse_posting(line:String) -> (String, Vec<u32>) {
    let x = line.find('\t').unwrap();
    let term = line[..x].to_string();
    let list = line[x+1..].split_whitespace()
        .map(|idx_str| idx_str.parse::<u32>())
        .filter_map(|res_idx| match res_idx {
            Ok(idx) => Option::Some(idx),
            Err(_)  => Option::None })
        .collect();
    (term, list)
}

fn parse_idx2doc(line:String) -> (u32, String) {
    let x = line.find('\t').unwrap();
    let idx = line[..x].parse::<u32>().unwrap();
    let doc = line[x+1..].to_string();
    (idx, doc)
}

pub trait SortedSet<T> {
    fn intersection(&self, s:&Self) -> Self;
}

impl<T> SortedSet<T> for Vec<T> where T:Ord+Copy {
    fn intersection(&self, other:&Self) -> Self {
        let mut inter = Vec::new();

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
