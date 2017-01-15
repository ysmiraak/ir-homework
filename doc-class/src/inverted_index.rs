use sparse_vec::{SparseVec, scale_to_unit};
use std::collections::HashMap;
use protocoll::MapMut;

pub type PostingsList = SparseVec<usize>;

pub struct InvertedIndex {
    inv_idx: HashMap<usize, PostingsList>,
    doc_count: usize,
}

impl InvertedIndex {
    pub fn new() -> Self {
        InvertedIndex {
            inv_idx: HashMap::new(),
            doc_count: 0,
        }
    }

    pub fn inv_insert(&mut self, doc: usize, term: usize) {
        if self.doc_count <= doc {
            self.doc_count = doc + 1
        }
        self.inv_idx
            .entry(term)
            .or_insert(PostingsList::new())
            .update_mut(doc, 0, |n| *n += 1)
    }

    pub fn inv_push<I>(&mut self, terms: I)
        where I: Iterator<Item = usize>
    {
        let doc = self.doc_count;
        for term in terms {
            self.inv_insert(doc, term)
        }
    }

    /// returns a document feature matrix;
    /// `feat_fn` shoud compute a feature from `(term_freq, doc_freq, doc_count)`;
    /// terms under `min_freq` are ignored.
    pub fn doc_features(&self,
                        feat_fn: fn(usize, usize, usize) -> f32,
                        min_freq: usize)
                        -> Vec<SparseVec<f32>> {
        let dc = self.doc_count;
        let mut feat_mat = Vec::new();
        feat_mat.resize(dc, SparseVec::new());
        let mut dim = 0;
        for doc2tf in self.inv_idx.values() {
            let df = doc2tf.len();
            if df < min_freq {
                continue;
            }
            for &(doc, tf) in doc2tf {
                feat_mat[doc].insert(dim, feat_fn(tf, df, dc));
            }
            dim += 1;
        }
        for feat_vec in &mut feat_mat {
            scale_to_unit(feat_vec)
        }
        println!("actual total dim: {}", dim);
        feat_mat
    }
}

pub fn binary(tf: usize, _: usize, _: usize) -> f32 {
    if tf > 0 { 1.0 } else { 0.0 }
}

pub fn tf_idf(tf: usize, df: usize, dc: usize) -> f32 {
    tf as f32 * idf(dc, df)
}

pub fn btf_idf(tf: usize, df: usize, dc: usize) -> f32 {
    binary_tf(tf) * idf(dc, df)
}

pub fn stf_idf(tf: usize, df: usize, dc: usize) -> f32 {
    sublinear_tf(tf) * idf(dc, df)
}

pub fn idf(dc: usize, df: usize) -> f32 {
    f32::ln(dc as f32 / df as f32)
}

pub fn binary_tf(tf: usize) -> f32 {
    if tf > 0 { 1.0 } else { 0.0 }
}

pub fn sublinear_tf(tf: usize) -> f32 {
    if tf > 0 {
        1.0 + f32::ln(tf as f32)
    } else {
        0.0
    }
}
