use std::cmp::min;
use std::fs::File;
use std::io::{BufReader,BufRead,stdin};
use std::collections::HashMap;
use std::hash::Hash;

trait Posting<T:Posting<T>> {
    fn intersect(&self, other: &T) -> T;
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

fn parse_into_map<K,V>(file:File, f:&Fn(String) -> (K,V)) -> HashMap<K,V>
    where K:Hash+Eq {
    BufReader::new(file).lines()
        .filter_map(|line_res| match line_res {
            Ok(line) => Option::Some(f(line)),
            Err(_) => Option::None })
        .fold(HashMap::new(), |mut m,(k,v)| { m.insert(k,v); m })
}

fn main() {
    let index_file_name = "index.txt";
    let table_file_name = "tubadw-r1-ir-ids-1000.tab";

    let index_in = File::open(index_file_name).unwrap();
    let table_in = File::open(table_file_name).unwrap();

    let lem2idx = parse_into_map(index_in, &|line| {
        let x = line.find('\t').unwrap();
        (line[..x].to_string(),
         line[x+1..].split_whitespace()
         .map(|idx_str| idx_str.parse::<usize>())
         .filter_map(|idx_res| match idx_res {
             Ok(idx) => Option::Some(idx),
             Err(_) => Option::None })
         .collect::<Vec<usize>>())});

    let idx2doc = parse_into_map(table_in, &|line| {
        let x = line.find('\t').unwrap();
        (line[..x].parse::<usize>().unwrap(), line[x+1..].to_string())});

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
        terms.sort_by(|a,b| a.len().cmp(&b.len()));
        for idx in terms[1..].iter().fold(terms[0].clone(), |a,b| a.intersect(b)) {
            println!("{}: {}", idx, idx2doc.get(&idx).unwrap());
        }
        println!("\n\nenter query:");
    }
}
