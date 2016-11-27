extern crate wildcard;
extern crate rand;

use rand::{thread_rng,Rng};
use wildcard::trie::{Trie,TernaryTrie};
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};


fn main() {
    let args:Vec<String> = args().collect();
    if 2 != args.len() {println!("usage: {} WORD_LIST_FILE",args[0]); exit(1)}
    
    let mut words:Vec<String> = match File::open(&args[1]) {
        Err(_) => { println!("cannot open file for reading: {}",args[1]); exit(2)}
        Ok(file) => BufReader::new(file).lines()
            .map(|res_w| res_w.unwrap())
            .collect()
    };
    
    let mut rng = thread_rng();
    rng.shuffle(&mut words);

    let (t1,t2) = words.into_iter().fold
        ((TernaryTrie::new(),TernaryTrie::new()),
         |(t1,t2),w| (t1.learn(w.chars()),t2.learn(w.chars().rev())));
    
    let stdin = stdin();
    println!("enter query:");
    for res_line in stdin.lock().lines() {
        let line = match res_line {
            Err(err) => {println!("error: {}",err); continue},
            Ok(line) => line.trim().to_owned()
        };
        if line.is_empty() {continue}
        for word in t1.prefix_search(line.chars()) {
            println!("prefix: {}",word)
        }
        for word in t2.prefix_search(line.chars().rev()) {
            println!("suffix: {}",word)
        }
        println!("\n\nenter query:");
    }
}
