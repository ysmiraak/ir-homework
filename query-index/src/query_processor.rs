use protocoll::{Map, Seq};
use protocoll::map::VecSortedMap;
use sparse_dense_vec::DenseVec;
use inverted_index::{InvertedIndex, PostingList};
use ordered_float::NotNaN;
use std::iter::repeat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn dot(v1: &[f64], v2: &[f64]) -> f64 {
    v1.iter().zip(v2.iter()).map(|(x1, x2)| x1 * x2).sum()
}

pub fn identity_tf(tf: usize) -> f64 {
    tf as f64
}

pub fn binary_tf(tf: usize) -> f64 {
    if tf > 0 { 1.0 } else { 0.0 }
}

pub fn sublinear_tf(tf: usize) -> f64 {
    if tf > 0 {
        1.0 + f64::ln(tf as f64)
    } else {
        0.0
    }
}

pub struct QueryProcessor<'a,'b> {
    weight_tf: &'b Fn(usize) -> f64,
    inv_index: &'a InvertedIndex,
    doc_norms: DenseVec<f64>
}

impl<'a,'b> QueryProcessor<'a,'b> {
    pub fn new<F>(inv_index: &'a InvertedIndex, doc_count: usize, weight_tf: &'b F) -> Self
        where F: Fn(usize) -> f64
    {
        let ln_dc = f64::ln(doc_count as f64);
        QueryProcessor {
            weight_tf: weight_tf,
            inv_index: inv_index,
            doc_norms: inv_index.view_content().iter()
                .flat_map(|(_, doc2tf)| {
                    let ln_df = f64::ln(doc2tf.len() as f64);
                    doc2tf.iter().zip(repeat(ln_df))
                        .map(|(&(doc, tf), ln_df)| {
                            let tfidf = weight_tf(tf) * (ln_dc - ln_df);
                            (doc, tfidf * tfidf)
                        })
                }).collect::<DenseVec<_>>()
                .update_all(f64::sqrt)
        }
    }

    pub fn idf(&self, term: &str) -> f64 {
        match self.inv_index.get(term) {
            Some(doc2tf) => f64::ln(self.doc_norms.len() as f64 / doc2tf.len() as f64),
            None => -0.0
        }
    }

    pub fn _tfidf(&self, tf:usize, idf:f64) -> f64 {
        (self.weight_tf)(tf) * idf
    }

    pub fn process(&self, query: &[String]) -> BinaryHeap<DocSim> {
        let dummy = PostingList::new();
        let (idfs, q_vec, mut doc_tf_iters) = query.iter()
            .fold(VecSortedMap::new(), // frequencies of the query terms
                  |t2tf, t| t2tf.update(t, |opt_tf| 1 + opt_tf.unwrap_or(0))).iter()
            .fold((Vec::new(), Vec::new(), Vec::new()),
                  |(idfs, q_vec, doc_tf_iters), &(t, tf)| {
                      let idf = self.idf(t);
                      (idfs.inc(idf),
                       q_vec.inc(self._tfidf(tf, idf)),
                       doc_tf_iters.inc(self.inv_index.get(t).unwrap_or(&dummy).iter().peekable()))
                  });
        let q_norm = f64::sqrt(dot(&q_vec, &q_vec));
        let mut ret = BinaryHeap::new();
        loop {
            let mut opt_doc = None;
            for doc_tf_iter in &mut doc_tf_iters {
                match (opt_doc, doc_tf_iter.peek()) {
                    (None, Some(&&(d, _))) => opt_doc = Some(d),
                    (Some(doc), Some(&&(d, _))) => if d < doc { opt_doc = Some(d)},
                    _ => ()
                }
            }
            let doc = match opt_doc {
                Some(doc) => doc,
                None => break
            };
            let mut d_vec = Vec::new();
            for (doc_tf_iter, &idf) in doc_tf_iters.iter_mut().zip(idfs.iter()) {
                match doc_tf_iter.peek() {                    
                    Some(&&(d, tf)) => 
                        if d == doc {
                            doc_tf_iter.next();
                            d_vec.push(self._tfidf(tf, idf))
                        } else {
                            d_vec.push(0.0)
                        },
                    None => d_vec.push(0.0)
                }
            }
            let d_norm = match self.doc_norms.get(doc) {
                Some(&norm) => norm,
                None => 0.0
            };
            ret.push(DocSim::new(doc, dot(&q_vec, &d_vec) / (q_norm * d_norm)));
        }
        ret
    }
}

#[derive(Debug,Default,Clone,PartialEq,Eq,Hash)]
pub struct DocSim {
    sim: NotNaN<f64>,
    doc: usize
}

impl DocSim {
    pub fn new(doc: usize, sim: f64) -> DocSim {
        DocSim {
            sim: NotNaN::new(sim).unwrap_or(NotNaN::new(0.0).unwrap()),
            doc: doc
        }
    }

    pub fn doc(&self) -> usize {
        self.doc
    }

    pub fn sim(&self) -> f64 {
        *self.sim.as_ref()
    }
}

impl PartialOrd for DocSim {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sim.partial_cmp(&other.sim)
    }
}

impl Ord for DocSim {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sim.cmp(&other.sim)
    }
}
