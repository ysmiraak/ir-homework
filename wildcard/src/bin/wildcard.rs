extern crate wildcard;
extern crate rand;
extern crate heapsize;

use wildcard::trie::{Trie,ArrayMapTrie,BTreeMapTrie,HashMapTrie,TernaryTrie};
use wildcard::query::wildcard_query;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};
use rand::{thread_rng,Rng};
use heapsize::HeapSizeOf;

const USAGE:&'static str = "WORD_LIST_FILE [TRIE_TYPE]
available trie types: array, btree, hash, ternary (default).

";

fn main() {
    let mut args:Vec<String> = args().collect();
    match args.len() {
        2 => args.push("ternary".to_string()),
        3 => args[2] = args[2].to_lowercase(),
        _ => {println!("usage: {} {}",args[0],USAGE); exit(1)}
    }
    let words = match File::open(&args[1]) {
        Err(_) => {println!("cannot open file for reading: {}",args[1]); exit(2)}
        Ok(file) => BufReader::new(file).lines().filter_map(Result::ok)
    };
    match args[2].as_ref() {
        "array" => {
            let (mut t, mut r) = load_tries(words,ArrayMapTrie::new());
            print_sizes(&t,&r);
            t.shrink_to_fit();
            r.shrink_to_fit();
            println!("shrinking the tries ...");
            print_sizes(&t,&r);
            do_query(&t,&r)
        }
        "btree" => {
            let (t,r) = load_tries(words,BTreeMapTrie::new());
            print_sizes(&t,&r);
            do_query(&t,&r)
        }
        "hash" => {
            let (mut t, mut r) = load_tries(words,HashMapTrie::new());
            print_sizes(&t,&r);
            t.shrink_to_fit();
            r.shrink_to_fit();
            println!("shrinking the tries ...");
            print_sizes(&t,&r);
            do_query(&t,&r)
        }
        "ternary" => {
            let mut words:Vec<_> = words.collect();
            let mut rng = thread_rng();
            rng.shuffle(&mut words);
            let (t,r) = load_tries(words.into_iter(),TernaryTrie::new());
            print_sizes(&t,&r);
            do_query(&t,&r)
        }
        _ => {println!("unsupported trie type: {}",args[2]); exit(3)}
    }
}

fn load_tries<I,T>(words:I,zero:T) -> (T,T) where I:Iterator<Item = String>, T:Trie+Clone {
    words.fold((zero.clone(),zero), |(t,r),w| (t.learn(w.chars()),r.learn(w.chars().rev())))
}

fn do_query<T>(t:&T,r:&T) where T:Trie {
    let stdin = stdin();
    println!("\n\nenter query:");
    for res_line in stdin.lock().lines() {
        let line = match res_line {
            Err(err) => {println!("error: {}",err); continue},
            Ok(line) => line.trim().to_owned()
        };
        if line.is_empty() {continue}
        match wildcard_query(t,r,&line) {
            Err(err) => println!("{}",err),
            Ok(words) => for w in words {println!("{}",w)}
        }
        println!("\n\nenter query:");
    }
}

fn print_sizes<T>(t:&T,r:&T) where T:HeapSizeOf {
    let st = t.heap_size_of_children() as f64 / 1024.0 / 1024.0;
    let sr = r.heap_size_of_children() as f64 / 1024.0 / 1024.0;
    println!("heap size of the trie: {:.2}MB", st);
    println!("heap size of the reversed trie: {:.2}MB", sr);
    println!("total heap sizes: {:.2}MB", st + sr);
}
