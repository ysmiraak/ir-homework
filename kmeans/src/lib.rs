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

/// iterativelly recompute the `centroids` til convergence (down to `epsilon`),
/// or til `max_iter` is reached. rows in `data` should be normalized unit
/// vectors. when `data` has shape `(_, d)`, `centroids` has shape `(k, d)`.
pub fn kmeans<S>(data: &Matrix<S>,
                 mut centroids: Array2<f32>,
                 epsilon: f32, max_iter: usize, verbose: bool) -> Array2<f32>
    where S: Data<Elem = f32>
{
    let total_rows = data.rows() as isize;
    let batch_size = (500 * 1024 * 1024 * 8) / (32 * centroids.rows()) as isize;
    // batched processing is much faster than iterating through the rows, and
    // also limits the additional memory usage to 500 mb here.
    for i in 0..max_iter {
        if verbose { println!("iteration {} ...", i+1);}
        let new_centroids = {
            let mut new_centroids = Array2::zeros(centroids.dim());
            let mut i = 0;
            while i < total_rows {
                let j = min(i + batch_size, total_rows);
                let batch = data.slice(s![i..j, ..]);
                for (&i, v) in centroids
                    .dot(&batch.t())
                    .map_axis(Axis(0), |v| arg_max(&v))
                    .into_iter()
                    .zip(batch.outer_iter())
                { new_centroids.row_mut(i) + &v;}
                i = j;
            }
            for ref mut v in new_centroids.outer_iter_mut() {
                normalize(v);
            }
            new_centroids
        };
        if verbose {
            let mut diff = centroids.outer_iter()
                .zip(new_centroids.outer_iter())
                .map(|(v1, v2)| distance(&v1, &v2))
                .collect::<Array1<f32>>();
            let max = diff[arg_max(&diff)];
            let mean = diff.mean(Axis(0));
            diff -= &mean;
            let var = (diff.dot(&diff) / (diff.len() - 1) as f32).sqrt();
            println!("shift of centroids:\tmax\t{}\tmean\t{}\tvar\t{}", max, mean, var);
        }
        if centroids.all_close(&new_centroids, epsilon) {
            return new_centroids;
        } else {
            centroids = new_centroids;
        }
    }
    centroids
}

/// seeing `v: index -> value`, returns the `index` which maximizes `value`, or
/// `0` for an empty `v`.
pub fn arg_max<S>(v: &Vector<S>) -> usize where S: Data<Elem = f32> {
    v.fold((0, 0, f32::NEG_INFINITY),
           |(m, i, x), &y|
           if x < y {
               (i, i+1, y)
           } else {
               (m, i+1, x)
           }).0
}

/// scales `v` into a unit vector, or do nothing if `v` is a null vector.
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
