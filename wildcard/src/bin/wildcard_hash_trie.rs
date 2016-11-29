extern crate wildcard;

use wildcard::trie::{Trie,HashMapTrie};
use wildcard::query::wildcard_query;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};

fn main() {
    let args:Vec<String> = args().collect();
    if 2 != args.len() {println!("usage: {} WORD_LIST_FILE",args[0]); exit(1)}
    let file = match File::open(&args[1]) {
        Err(_) => {println!("cannot open file for reading: {}",args[1]); exit(2)}
        Ok(file) => BufReader::new(file)};

    let (mut t, mut r) = file.lines().filter_map(Result::ok)
        .fold((HashMapTrie::new(),HashMapTrie::new()),
              |(t,r),w| (t.learn(w.chars()),r.learn(w.chars().rev())));

    t.shrink_to_fit();
    r.shrink_to_fit();
    
    let stdin = stdin();
    println!("enter query:");
    for res_line in stdin.lock().lines() {
        let line = match res_line {
            Err(err) => {println!("error: {}",err); continue},
            Ok(line) => line.trim().to_owned()
        };
        if line.is_empty() {continue}
        match wildcard_query(&t,&r,&line) {
            Err(err) => println!("{}",err),
            Ok(words) => for w in words {println!("{}",w)}
        }
        println!("\n\nenter query:");
    }
}
