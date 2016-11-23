#![feature(libc)]
extern crate libc;
use libc::*;
extern {fn je_malloc_stats_print (write_cb: extern fn (*const c_void, *const c_char), cbopaque: *const c_void, opts: *const c_char);}
extern fn write_cb (_: *const c_void, message: *const c_char) {
    print! ("{}", String::from_utf8_lossy (unsafe {std::ffi::CStr::from_ptr (message as *const i8) .to_bytes()}));
}

extern crate protocoll;

use std::env::args;
use std::process;
use std::fs::File;
use std::io::{BufReader,BufRead};
use protocoll::{Map,Str,Seq};
use std::collections::HashMap;

pub trait Trie {
    fn learn(self, s:&str) -> Self;
    fn recognize(&self, s:&str) -> bool;
    fn prefix_search(&self, prefix:&str) -> Vec<String>;
}

#[derive(Debug)]
pub enum MapTrie {
    Accept(HashMap<char,MapTrie>),
    NonAcc(HashMap<char,MapTrie>)
}

impl Default for MapTrie {
    fn default() -> Self {
        MapTrie::new()
    }
}

impl MapTrie {
    pub fn new() -> Self {
        MapTrie::NonAcc(HashMap::new())
    }

    pub fn is_accept(&self) -> bool {
        match self {
            &MapTrie::Accept(_) => true,
            &MapTrie::NonAcc(_) => false
        }
    }

    pub fn view_content(&self) -> &HashMap<char,MapTrie> {
        match self {
            &MapTrie::Accept(ref m) => m,
            &MapTrie::NonAcc(ref m) => m
        }
    }

    fn _learn<I>(self, mut s:I) -> Self where I:Iterator<Item = char> {
        match (self,s.next()) {
            (MapTrie::Accept(m),None) => MapTrie::Accept(m),
            (MapTrie::NonAcc(m),None) => MapTrie::Accept(m),
            (MapTrie::Accept(m),Some(c)) => MapTrie::Accept
                (m.update(c, |opt_t| opt_t.unwrap_or_default()._learn(s))),
            (MapTrie::NonAcc(m),Some(c)) => MapTrie::NonAcc
                (m.update(c, |opt_t| opt_t.unwrap_or_default()._learn(s)))
        }
    }

    fn _recognize<I>(&self, mut s:I) -> bool where I:Iterator<Item = char> {
        match s.next() {
            None => self.is_accept(),
            Some(c) => match self.view_content().get(&c) {
                None => false,
                Some(t) => t._recognize(s)
            }
        }
    }

    fn _prefix_search(&self, prefix:String) -> Vec<String> {
        self.view_content().iter()
            .fold(if self.is_accept() {Vec::new().inc(prefix.clone())} else {Vec::new()},
                  |ret,(c,t)| ret.plus(t._prefix_search(prefix.clone().inc(*c))))
    }
}

impl Trie for MapTrie {
    fn learn(self, s:&str) -> Self {
        self._learn(s.chars())
    }

    fn recognize(&self, s:&str) -> bool {
        self._recognize(s.chars())
    }

    fn prefix_search(&self, prefix:&str) -> Vec<String> {
        let mut s = prefix.chars();
        let mut t = self;
        loop {
            match s.next() {
                None => return t._prefix_search(String::from(prefix)),
                Some(c) => match t.view_content().get(&c) {
                    None => return Vec::new(),
                    Some(tc) => t = tc
                }
            }
        }
    }
}

fn main() {
    let args:Vec<String> = args().collect();

    // let print_title = match args.len() {
    //     2 => false,
    //     3 => true,
    //     _ => { println!("usage: {} TERM_INDEX_FILE (INDEX_TITLE_FILE)", args[0]); exit(1)}
    // };

    let trie = match File::open(&args[1]) {
        Err(_) => { println!("cannot open file for reading: {}", args[1]); process::exit(2)}
        Ok(file) => BufReader::new(file).lines().fold
            (MapTrie::new(), |t,s| t.learn(&s.unwrap()))
    };

    println!("{}",trie.view_content().len());


    unsafe {je_malloc_stats_print (write_cb, std::ptr::null(), std::ptr::null())};

    // let t = MapTrie::new()
    //     .learn("abc")
    //     .learn("abd")
    //     .learn("aaa")
    //     .learn("");
    // println!("{:?}", t);
    // println!("{:?}", t.recognize("abd"));
    // println!("{:?}", t.prefix_search("ab"));
}
