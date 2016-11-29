extern crate wildcard;
extern crate rand;

use wildcard::trie::{Trie,TernaryTrie};
use wildcard::query::wildcard_query;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};
use rand::{thread_rng,Rng};

fn main() {
    let args:Vec<String> = args().collect();
    if 2 != args.len() {println!("usage: {} WORD_LIST_FILE",args[0]); exit(1)}
    let file = match File::open(&args[1]) {
        Err(_) => {println!("cannot open file for reading: {}",args[1]); exit(2)}
        Ok(file) => BufReader::new(file)};

    let mut words:Vec<_> = file.lines().filter_map(Result::ok).collect();
    
    let mut rng = thread_rng();
    rng.shuffle(&mut words);

    let (t,r) = words.into_iter().fold
        ((TernaryTrie::new(),TernaryTrie::new()),
         |(t,r),w| (t.learn(w.chars()),r.learn(w.chars().rev())));
    
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
