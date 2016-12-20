use protocoll::{Map, Set, Seq};
use protocoll::map::VecSortedMap;
use sparse_dense_vec::DenseVec;
use inverted_index::{InvertedIndex, PostingList};
use ordered_float::NotNaN;
use std::iter::repeat;
use std::collections::{BinaryHeap, HashSet};

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

pub struct QueryProcessor<'a> {
    weight_tf: fn(usize) -> f64,
    inv_index: &'a InvertedIndex,
    doc_norms: DenseVec<f64>
}

impl<'a> QueryProcessor<'a> {
    pub fn new(inv_index: &'a InvertedIndex, doc_count: usize,
               weight_tf: fn(usize) -> f64) -> Self
    {
        let ln_dc = f64::ln(doc_count as f64);
        QueryProcessor {
            weight_tf: weight_tf,
            inv_index: inv_index,
            doc_norms: inv_index.view_content().values()
                .flat_map(|doc2tf| {
                    let ln_df = f64::ln(doc2tf.len() as f64);
                    doc2tf.iter().zip(repeat(ln_df))
                        .map(|(&(doc, tf), ln_df)| {
                            let _tfidf = weight_tf(tf) * (ln_dc - ln_df);
                            (doc, _tfidf * _tfidf)})})
                .fold(DenseVec::new(), |doc_sum_sqs, (doc, sq)| doc_sum_sqs
                      .update(doc, |opt_sum_sqs| sq + opt_sum_sqs.unwrap_or_default()))
                .update_all(f64::sqrt)
                .shrink()
        }
    }

    pub fn idf(&self, term: &str) -> f64 {
        match self.inv_index.get(term) {
            Some(doc2tf) => f64::ln(self.doc_norms.len() as f64 / doc2tf.len() as f64),
            None => -0.0
        }
    }

    pub fn weight_tf(&self, tf:usize) -> f64 {
        (self.weight_tf)(tf)
    }

    pub fn process(&self, query: &[String]) -> BinaryHeap<DocSim> {
        let dummy = PostingList::new();

        let (idfs, q_vec, doc2tf_vec, docs) = query.iter()
            .fold(VecSortedMap::new(), // frequencies of the query terms
                  |t2tf, t| t2tf.update(t, |opt_tf| 1 + opt_tf.unwrap_or(0))).iter()
            .fold((Vec::new(), Vec::new(), Vec::new(), HashSet::new()),
                  |(idfs, q_vec, doc2tf_vec, docs), &(t, tf)| {
                      let idf = self.idf(t);
                      let doc2tf = self.inv_index.get(t).unwrap_or(&dummy);
                      (idfs.inc(idf),
                       q_vec.inc(idf * self.weight_tf(tf)),
                       doc2tf_vec.inc(doc2tf),
                       docs.plus(doc2tf.iter().map(|&(d, _)| d)))
                  });

        let q_norm = f64::sqrt(dot(&q_vec, &q_vec));

        docs.iter().fold(BinaryHeap::new(), |ret, &doc| {
            let d_vec = doc2tf_vec.iter().zip(idfs.iter())
                .map(|(doc2tf, idf)| idf *
                     self.weight_tf(doc2tf.get(&doc).map(ToOwned::to_owned).unwrap_or_default()))
                .collect::<Vec<_>>();

            let d_norm = self.doc_norms.get(doc).map(ToOwned::to_owned).unwrap_or_default();

            ret.inc(DocSim::new(doc, dot(&q_vec, &d_vec) / (q_norm * d_norm)))
        })
    }
}

#[derive(Debug,Default,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
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
