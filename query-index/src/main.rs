//! Author: Kuan Yu, 3913893
//! Honor Code: I pledge that this program represents my own work.

use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};
use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Cow;

fn main() {
    // let args:Vec<&str> = vec!["query-index","index.txt","tubadw-r1-ir-ids-100000.tab"];
    
    let args:Vec<String> = args().collect();

    let print_title = match args.len() {
        2 => false,
        3 => true,
        _ => { println!("usage: {} TERM_INDEX_FILE (INDEX_TITLE_FILE)", args[0]); exit(1)}
    };

    let posting = match File::open(&args[1]) {
        Err(_) => { println!("cannot open file for reading: {}", args[1]); exit(2)}
        Ok(file) => parse_to_map(BufReader::new(file), parse_posting)
    };

    let idx2doc = if !print_title { HashMap::new() } else {
        match File::open(&args[2]) {
            Err(_) => { println!("cannot open file for reading: {}", args[2]); exit(3)},
            Ok(file) => parse_to_map(BufReader::new(file), parse_idx2doc)
        }
    };
    
    let stdin = stdin();
    println!("enter query:");
    'doquery: for res_line in stdin.lock().lines() {
        let line = match res_line {
            Err(err) => { println!("error: {}",err); continue 'doquery },
            Ok(line) => line
        };
        let mut terms = Vec::new();
        for term in line.split_whitespace() {
            if term.is_empty() { continue }
            match posting.get(term) {
                Some(p) => terms.push(p),
                None => { println!("no match found.\n\nenter query:"); continue 'doquery }
            }
        }
        if terms.is_empty() { continue 'doquery }
        // sort and do AND query
        terms.sort_by(|a,b| a.len().cmp(&b.len()));
        let posting_list = terms[1..].iter()
            .fold(Cow::Borrowed(terms[0]), |a,b| Cow::Owned((&a).intersection(b)))
            .into_owned();
        for idx in posting_list {
            match print_title {
                true => println!("{}: {}", idx, idx2doc.get(&idx).unwrap_or(&String::new())),
                false => println!("{}",idx)
            }
        }
        println!("\n\nenter query:");
    }
}

fn parse_to_map<F,K,V,R>(rdr:R, f:F) -> HashMap<K,V>
    where F:Fn(String) -> Option<(K,V)>,
          K:Hash+Eq,
          R:BufRead,{
    rdr.lines()
        .filter_map(|res_line| match res_line {
            Err(err) => { println!("error: {}",err); Option::None },
            Ok(line) => f(line)})
        .collect()
}

fn parse_posting(line:String) -> Option<(String, Vec<u32>)> {
    let x = match line.find('\t') {
        None => { println!("illformed: {}",line); return Option::None },
        Some(x) => x };
    let term = line[..x].to_string();
    let list = line[x+1..].split_whitespace()
        .map(|idx_str| idx_str.parse::<u32>())
        .filter_map(|res_idx| match res_idx {
            Err(_) => { println!("illformed: {}",line); Option::None },
            Ok(idx) => Option::Some(idx)})
        .collect();
    Option::Some((term,list))
}

fn parse_idx2doc(line:String) -> Option<(u32, String)> {
    let x = match line.find('\t') {
        None => { println!("illformed: {}",line); return Option::None },
        Some(x) => x };
    let idx = match line[..x].parse::<u32>() {
        Err(_) => { println!("illformed: {}",line); return Option::None },
        Ok(idx) => idx };
    let doc = line[x+1..].to_string();
    Option::Some((idx,doc))
}

pub trait SortedSet<T> {
    fn intersection(&self, s:&Self) -> Self;
}

impl<T> SortedSet<T> for Vec<T> where T:Ord+Copy {
    /// author: [danieldk](https://github.com/danieldk/ir-examples)
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
