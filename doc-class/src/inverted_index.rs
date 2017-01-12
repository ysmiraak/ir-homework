use sparse_vec::SparseVec;
use protocoll::MapMut;
use std::collections::HashMap;
use std::f64;

pub type PostingList = SparseVec<usize>;

#[derive(Debug,Default,Clone,PartialEq,Eq)]
pub struct InvertedIndex {
    inv_idx: HashMap<String, PostingList>,
    num_doc: usize
}

impl InvertedIndex {
    pub fn new() -> InvertedIndex {
        InvertedIndex {
            inv_idx: HashMap::new(),
            num_doc: 0
        }
    }
    
    pub fn add_doc<I>(self, doc: I) -> Self where I: Iterator<Item = String> {
        let n = self.num_doc;

        InvertedIndex {
            inv_idx: doc.fold(self.inv_idx, |mut m, s| {
                m.update_mut(s, PostingList::new(), |p| p.update_mut(n, 0, |c| *c += 1));
                m
            }),
            num_doc: n + 1
        }
    }

    pub fn shrink(mut self) -> Self {
        self.inv_idx.update_all_mut(|_, p| p.shrink_to_fit());
        self.inv_idx.shrink_to_fit();
        self
    }

    pub fn view_content(&self) -> &HashMap<String, PostingList> {
        &self.inv_idx
    }

    pub fn features(&self) -> Vec<SparseVec<f64>> {
        let mut doc_feats = Vec::new();
        for _ in 0..self.num_doc {
            doc_feats.push(SparseVec::new())
        }
        let mut idx = 0;
        for (_, doc2tf) in &self.inv_idx {
            let idf = f64::ln(self.num_doc as f64 / doc2tf.len() as f64);
            if idf < 0.1 || 9.0 < idf { continue }
            for &(doc, tf) in doc2tf {
                doc_feats.get_mut(doc).unwrap().insert(idx, tf as f64 * idf);
            }
            idx += 1;
        }
        println!("term count: {}", idx);
        // let mut max = Vec::new();
        // for _ in 0..self.inv_idx.len() {
        //     max.push(0.0)
        // }
        // for idx2feat in &doc_feats {
        //     for &(idx, feat) in idx2feat {
        //         if feat > max[idx] {
        //             max[idx] = feat
        //         }
        //     }
        // }
        // for idx2feat in &mut doc_feats {
        //     idx2feat.update_all_mut(|&idx, feat| *feat = *feat / max[idx])
        // }
        doc_feats
    }
}

// pub fn dot(v1: &[f64], v2: &[f64]) -> f64 {
//     v1.iter().zip(v2.iter()).map(|(x1, x2)| x1 * x2).sum()
// }

// pub fn identity_tf(tf: usize) -> f64 {
//     tf as f64
// }

// pub fn binary_tf(tf: usize) -> f64 {
//     if tf > 0 { 1.0 } else { 0.0 }
// }

// pub fn sublinear_tf(tf: usize) -> f64 {
//     if tf > 0 {
//         1.0 + f64::ln(tf as f64)
//     } else {
//         0.0
//     }
// }
