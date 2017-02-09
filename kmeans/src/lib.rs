// Author: Kuan Yu, 3913893
// Honor Code:  I pledge that this program represents my own work.

#[macro_use(s)]
extern crate ndarray;

use std::f32;
use std::cmp::min;
use ndarray::{ArrayBase, Array, Array1, Array2, Axis, RemoveAxis, Ix1, Ix2, Data, DataMut};

/// ArrayBase1
pub type Vector<S> = ArrayBase<S, Ix1>;

/// ArrayBase2
pub type Matrix<S> = ArrayBase<S, Ix2>;

/// deterministically select `n` samples from `data` along the outer-most axis.
pub fn step_sample<A, S, D>(data: &ArrayBase<S, D>, n: usize) -> Array<A, D>
    where A: Copy, S: Data<Elem = A>, D: RemoveAxis
{
    let step = data.shape()[0] / n;
    let mut sel = Vec::new();
    for i in 0..n {
        sel.push(i * step);
    }
    data.select(Axis(0), &sel)
}

// /// todo
// pub fn randome_sample<A, S, D>(data: &ArrayBase<S, D>, n: usize, seed: usize) -> Array<A, D>
//     where A: Copy, S: Data<Elem = A>, D: RemoveAxis

/// iterativelly recompute the `centroids` til convergence (with `epsilon` error
/// tolerance), or until `iter_max` is reached. rows in `data` should be
/// normalized unit vectors. the shape of `data` be `(_, d)`, then the shape of
/// `centroids` be `(k, d)`.
pub fn kmeans<S>(data: &Matrix<S>,
                 mut centroids: Array2<f32>,
                 epsilon: f32, iter_max: usize, verbose: bool) -> Array2<f32>
    where S: Data<Elem = f32>
{
    let (k, d) = centroids.dim();
    let n = data.rows() as isize;
    let b = (250 * 1024 * 1024 * 8) / (32 * k) as isize;
    // batched processing, much faster than going through each row separately,
    // but also limit the additional memory usage to 250 mb here.
    for i in 0..iter_max {
        if verbose { println!("iteration {} ...", i+1);}
        let new_centroids = {
            let mut cb = CentroidBuilder::new(k, d);
            let mut i = 0;
            while i < n {
                let j = min(i + b, n);
                let batch = data.slice(s![i..j, ..]);
                for (i, v) in centroids
                    .dot(&batch.t())
                    .map_axis(Axis(0), |v| arg_max(&v))
                    .into_iter()
                    .zip(batch.outer_iter())
                { cb.inc(*i, &v);}
                i = j;
            }
            cb.build(verbose)
        };
        if verbose {
            let (mean, var) = {
                let mut diff = centroids.outer_iter()
                    .zip(new_centroids.outer_iter())
                    .map(|(v1, v2)| distance(&v1, &v2))
                    .collect::<Array1<f32>>();
                let mean = diff.scalar_sum() / diff.len() as f32;
                diff -= mean;
                (mean, (diff.dot(&diff) / (diff.len() - 1) as f32).sqrt())
            };
            println!("shift of centroids:\tmean\t{}\tvar\t{}", mean, var);
        }
        if centroids.all_close(&new_centroids, epsilon) {
            return new_centroids;
        } else {
            centroids = new_centroids;
        }
    }
    centroids
}

/// returns the index of the closest centroid in `centroids` for `data`.
pub fn settle_to<S1, S2>(data: &Vector<S1>, centroids: &Matrix<S2>) -> usize
    where S1: Data<Elem = f32>, S2: Data<Elem = f32> {
    arg_max(&centroids.dot(data))
}

/// seeing `v: index -> value`, returns the `index` which maximizes `value`, or
/// `0` for an empty `v`.
pub fn arg_max<S>(v: &Vector<S>) -> usize where S: Data<Elem = f32> {
    v.fold((0, 0, f32::NEG_INFINITY),
           |(m, i, x), y|
           if x < *y {
               (i, i+1, *y)
           } else {
               (m, i+1, x)
           }).0
}

/// scales `v` into a unit vector, or do nothing if `v` is a unit vector.
pub fn normalize<S>(v: &mut Vector<S>) where S: DataMut<Elem = f32> {
    let norm = v.dot(v).sqrt();
    if norm != 0.0 {
        *v /= norm;
    }
}

/// returns the euclidean distance between `v1` and `v2`.
pub fn distance<S1, S2>(v1: &Vector<S1>, v2: &Vector<S2>) -> f32
    where S1: Data<Elem = f32>,  S2: Data<Elem = f32> {
    let diff = v1 - v2;
    diff.dot(&diff).sqrt()
}

#[derive(Debug,Default,Clone,PartialEq)]
struct CentroidBuilder {
    acc: Array2<f32>,
    cnt: Array1<f32>
}

impl CentroidBuilder {
    fn new(k: usize, d: usize) -> Self {
        CentroidBuilder {
            acc: Array2::<f32>::zeros((k, d)),
            cnt: Array1::<f32>::zeros(k)
        }
    }

    fn inc<S>(&mut self, i: usize, v: &Vector<S>) where S: Data<Elem = f32> {
        self.cnt[i] += 1.0;
        self.acc.row_mut(i) + &v;
    }

    fn build(mut self, verbose: bool) -> Array2<f32> {
        for (mut v, (i, n)) in self.acc.outer_iter_mut().zip(self.cnt.indexed_iter()) {
            if *n == 0.0 {
                if verbose {
                    println!("cluster {} is uninhabited. its centroid is now the origin.", i)
                }
            } else {
                v /= *n;
                normalize(&mut v);
            }
        }
        self.acc
    }
}
