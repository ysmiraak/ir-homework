// Author: Kuan Yu, 3913893
// Honor Code:  I pledge that this program represents my own work.

use protocoll::map::VecSortedMap;
use protocoll::MapMut;

pub type SparseVec<T> = VecSortedMap<usize, T>;

pub fn scale_to_unit(v: &mut SparseVec<f32>) {
    let mut xx = 0.0;
    for &(_, x) in v.iter() {
        xx += x * x
    }
    let norm = f32::sqrt(xx);
    v.update_all_mut(|_, x| *x = *x / norm);
}
