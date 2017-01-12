extern crate protocoll;

pub mod error;
pub mod numberer;
pub mod inverted_index;

pub mod sparse_vec {
    use protocoll::map::VecSortedMap;
    pub type SparseVec<T> = VecSortedMap<usize, T>;
}
