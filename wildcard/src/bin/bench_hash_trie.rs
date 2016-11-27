// #![feature(libc)]
// extern crate libc;
// extern {fn je_stats_print(write_cb:extern fn (*const libc::c_void, *const libc::c_char), cbopaque:*const libc::c_void, opts:*const libc::c_char);}
// extern fn write_cb(_:*const libc::c_void, m:*const libc::c_char) {print!("{}",String::from_utf8_lossy(unsafe{std::ffi::CStr::from_ptr(m as *const i8).to_bytes()}));}

extern crate wildcard;

use wildcard::{Trie,HashMapTrie};
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};

fn main() {
    let args:Vec<String> = args().collect();
    if 2 != args.len() {println!("usage: {} WORD_LIST_FILE",args[0]); exit(1)}

    let trie = match File::open(&args[1]) {
        Err(_) => { println!("cannot open file for reading: {}",args[1]); exit(2)}
        Ok(file) => BufReader::new(file).lines().fold
            (HashMapTrie::new(), |t,s| t.learn(&s.unwrap()))
    };

    println!("alphabets: {}",trie.view_content().len());

    let stdin = stdin();
    for res_line in stdin.lock().lines() {
        match res_line {
            Err(err) => {println!("error: {}",err)},
            Ok(line) => {println!("{}",line)}
        }
    }

    // unsafe{je_stats_print(write_cb,std::ptr::null(),std::ptr::null())}
}
