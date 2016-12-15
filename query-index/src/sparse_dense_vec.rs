use protocoll::map::VecSortedMap;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::iter::FromIterator;
use std::mem::replace;

pub trait Dimension {
    fn dim(&self) -> usize;
}

pub type SparseVec<T> = VecSortedMap<usize, T>;

impl<T> Dimension for SparseVec<T> {
    fn dim(&self) -> usize {
        match self.view_content().last() {
            Some(&(i, _)) => i,
            None => 0
        }
    }
}

/// a vec that's useful for indexing, but allows for missing elements.
#[derive(Debug,Default,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct DenseVec<T> {
    vec: Vec<Option<T>>,
    len: usize
}

impl<T> DenseVec<T> {
    pub fn view_content(&self) -> &[Option<T>] {
        &self.vec
    }

    /// # example
    /// ```
    /// use query_index::sparse_dense_vec::{DenseVec,Dimension};
    /// let mut dv = DenseVec::new();
    /// dv.insert(1,1);
    /// dv.insert(4,4);
    /// dv.insert(3,3);
    /// assert_eq!(dv.len(), 3);
    /// assert_eq!(dv.get(0), None);
    /// assert_eq!(dv.get(1), Some(&1));
    /// assert_eq!(dv.get(2), None);
    /// assert_eq!(dv.get(3), Some(&3));
    /// assert_eq!(dv.get(4), Some(&4));
    /// assert_eq!(dv.dim(), 5);
    /// ```
    pub fn new() -> Self {
        DenseVec {
            vec: Vec::new(),
            len: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// ensures that the `i`-th element is `None`, and that `dim` of `self` is at
    /// least `i+1`. returns the old element at `i` if any.
    pub fn make_space(&mut self, i: usize) -> Option<T> {
        match self.vec.len().cmp(&i) {
            Less => {
                while self.vec.len() <= i {
                    self.vec.push(None)
                }
                None
            }
            Equal => {
                self.vec.push(None);
                None
            }
            Greater => {
                let old = replace(&mut self.vec[i], None);
                if old.is_some() {
                    self.len -= 1
                }
                old
            }
        }
    }

    pub fn insert(&mut self, i: usize, e: T) -> Option<T> {
        let old = self.make_space(i);
        replace(&mut self.vec[i], Some(e));
        self.len += 1;
        old
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        match self.vec.len().cmp(&i) {
            Greater => self.make_space(i),
            _ => None,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self.vec.get(i) {
            Some(ref_opt_e) => {
                match ref_opt_e {
                    &Some(ref e) => Some(e),
                    &None => None
                }
            }
            None => None
        }
    }

    /// # example
    /// ```
    /// use query_index::sparse_dense_vec::{DenseVec,Dimension};
    /// let mut dv = DenseVec::new();
    /// dv.insert(1,1);
    /// dv.insert(4,4);
    /// dv.insert(3,3);
    /// assert_eq!(dv.get(0), None);
    /// assert_eq!(dv.len(), 3);
    /// assert_eq!(dv.dim(), 5);
    /// let dv = dv.update(0, |opt_e| opt_e.unwrap_or(0));
    /// assert_eq!(dv.get(0), Some(&0));
    /// assert_eq!(dv.len(), 4);
    /// assert_eq!(dv.dim(), 5);
    /// ```
    pub fn update<F>(mut self, i: usize, f: F) -> Self
        where F: FnOnce(Option<T>) -> T
    {
        let old = self.make_space(i);
        replace(&mut self.vec[i], Some(f(old)));
        self.len += 1;
        self
    }

    /// # example
    /// ```
    /// use query_index::sparse_dense_vec::{DenseVec,Dimension};
    /// let mut dv = DenseVec::new();
    /// dv.insert(1,1);
    /// dv.insert(4,4);
    /// dv.insert(3,3);
    /// let dv = dv.update(0, |opt_e| opt_e.unwrap_or(0));
    /// let dv = dv.update_all(|n| n + 1);
    /// assert_eq!(dv.get(0), Some(&1));
    /// assert_eq!(dv.get(1), Some(&2));
    /// assert_eq!(dv.get(2), None);
    /// assert_eq!(dv.get(3), Some(&4));
    /// assert_eq!(dv.get(4), Some(&5));
    /// assert_eq!(dv.len(), 4);
    /// assert_eq!(dv.dim(), 5);
    /// ```
    pub fn update_all<F>(mut self, mut f: F) -> Self
        where F: FnMut(T) -> T
    {
        for ref_opt_e in &mut self.vec {
            match replace(ref_opt_e, None) {
                Some(e) => replace(ref_opt_e, Some(f(e))),
                None => None
            };
        }
        self
    }

    pub fn shrink(mut self) -> Self {
        self.vec.shrink_to_fit();
        self
    }
}

impl<T> FromIterator<(usize, T)> for DenseVec<T> {
    fn from_iter<I>(iter: I) -> DenseVec<T>
        where I: IntoIterator<Item = (usize, T)>
    {
        let mut ret = DenseVec::new();
        for (i, e) in iter {
            ret.insert(i, e);
        }
        ret
    }
}

impl<T> Dimension for DenseVec<T> {
    fn dim(&self) -> usize {
        self.vec.len()
    }
}
